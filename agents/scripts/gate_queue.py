#!/usr/bin/env python3
"""FIFO queue for serialized cargo gate execution."""

from __future__ import annotations

import argparse
import fcntl
import json
import os
import shlex
import signal
import subprocess
import sys
import time
from contextlib import contextmanager
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from agent_io import parse_metadata


ROOT = Path(__file__).resolve().parents[2]
GATES_DIR = ROOT / "agents/status/gates"
QUEUE_DIR = GATES_DIR / "queue"
PENDING_FILE = QUEUE_DIR / "pending.jsonl"
RUNNING_FILE = QUEUE_DIR / "running.json"
HISTORY_DIR = QUEUE_DIR / "history"
RUNNER_LOCK = ROOT / ".agent_gate_queue.lock"
DATA_LOCK = ROOT / ".agent_gate_queue_data.lock"

CARGO_GATE_SCRIPT = ROOT / "agents/scripts/cargo_gate.sh"
EVAL_SCRIPT = ROOT / "agents/scripts/evaluate_full_test_failures.py"
BASELINE_FILE = ROOT / "agents/status/gates/baseline_failures.json"

MODE_TICKET = "TICKET"
MODE_MERGE = "MERGE"
VALID_MODES = {MODE_TICKET, MODE_MERGE}

CURRENT_CHILD: subprocess.Popen[str] | None = None
RECEIVED_SIGNAL: int | None = None


class GateInterrupted(Exception):
    """Raised when a gate step is interrupted by signal."""

    def __init__(self, step: dict[str, Any]):
        super().__init__("gate-step-interrupted")
        self.step = step


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def install_signal_handlers() -> None:
    def _handler(signum: int, _frame) -> None:
        global RECEIVED_SIGNAL, CURRENT_CHILD
        RECEIVED_SIGNAL = signum
        if CURRENT_CHILD and CURRENT_CHILD.poll() is None:
            CURRENT_CHILD.terminate()
        raise KeyboardInterrupt()

    signal.signal(signal.SIGINT, _handler)
    signal.signal(signal.SIGTERM, _handler)
    signal.signal(signal.SIGHUP, _handler)


def ensure_dirs() -> None:
    GATES_DIR.mkdir(parents=True, exist_ok=True)
    QUEUE_DIR.mkdir(parents=True, exist_ok=True)
    HISTORY_DIR.mkdir(parents=True, exist_ok=True)
    if not PENDING_FILE.exists():
        PENDING_FILE.write_text("", encoding="utf-8")
    if not RUNNING_FILE.exists():
        RUNNING_FILE.write_text("{}\n", encoding="utf-8")
    if not BASELINE_FILE.exists():
        BASELINE_FILE.parent.mkdir(parents=True, exist_ok=True)
        BASELINE_FILE.write_text(
            json.dumps(
                {
                    "known_failures": [],
                    "notes": "Known pre-existing full-test failures eligible for merge waiver.",
                },
                indent=2,
                ensure_ascii=True,
            )
            + "\n",
            encoding="utf-8",
        )


@contextmanager
def file_lock(path: Path, wait: bool = True):
    path.parent.mkdir(parents=True, exist_ok=True)
    fd = path.open("a+")
    try:
        if wait:
            fcntl.flock(fd.fileno(), fcntl.LOCK_EX)
        else:
            try:
                fcntl.flock(fd.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError:
                raise RuntimeError(f"lock-busy:{path}")
        yield
    finally:
        try:
            fcntl.flock(fd.fileno(), fcntl.LOCK_UN)
        finally:
            fd.close()


def load_pending() -> list[dict[str, Any]]:
    if not PENDING_FILE.exists():
        return []
    jobs: list[dict[str, Any]] = []
    for line in PENDING_FILE.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        jobs.append(json.loads(line))
    return jobs


def save_pending(jobs: list[dict[str, Any]]) -> None:
    text = "\n".join(json.dumps(j, ensure_ascii=True) for j in jobs)
    if text:
        text += "\n"
    PENDING_FILE.write_text(text, encoding="utf-8")


def set_running(job: dict[str, Any]) -> None:
    RUNNING_FILE.write_text(json.dumps(job, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")


def clear_running() -> None:
    RUNNING_FILE.write_text("{}\n", encoding="utf-8")


def history_path(job_id: str) -> Path:
    return HISTORY_DIR / f"{job_id}.json"


def make_job_id(mode: str, ticket_id: str) -> str:
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    ticket = ticket_id.replace("/", "_").replace(" ", "_")
    suffix = f"{os.getpid()}_{int(time.time() * 1000) % 1000000}"
    return f"{stamp}_{mode.lower()}_{ticket}_{suffix}"


def current_branch(workdir: Path) -> str:
    try:
        result = subprocess.run(
            ["git", "-C", str(workdir), "branch", "--show-current"],
            check=False,
            capture_output=True,
            text=True,
        )
        branch = result.stdout.strip()
        return branch or "unknown"
    except Exception:
        return "unknown"


def parse_scoped_test_command(raw: str) -> list[str]:
    tokens = shlex.split(raw.strip())
    if not tokens:
        raise ValueError("empty scoped test command")
    if tokens[0] in {"cargo-safe", "cargo"}:
        tokens = tokens[1:]
    if not tokens:
        raise ValueError("scoped test command missing cargo subcommand")
    if tokens[0] != "test":
        raise ValueError(f"scoped test command must start with `test`, got `{tokens[0]}`")
    return tokens


def scoped_test_from_ticket(ticket_id: str) -> str | None:
    ticket = ROOT / "agents/backlog" / f"{ticket_id}.md"
    if not ticket.exists():
        return None
    meta = parse_metadata(ticket.read_text(encoding="utf-8"))
    return meta.get("scoped_test_command")


def enqueue_job(args: argparse.Namespace) -> int:
    ensure_dirs()
    mode = args.mode.upper()
    if mode not in VALID_MODES:
        raise ValueError(f"unsupported mode: {mode}")

    workdir = Path(args.workdir).resolve()
    if not workdir.exists():
        raise ValueError(f"workdir does not exist: {workdir}")

    scoped_cmd = args.scoped_test
    if mode == MODE_TICKET and not scoped_cmd:
        scoped_cmd = scoped_test_from_ticket(args.ticket) or "cargo-safe test --lib"

    job_id = make_job_id(mode, args.ticket)
    job = {
        "job_id": job_id,
        "ticket_id": args.ticket,
        "mode": mode,
        "workdir": str(workdir),
        "branch": args.branch or current_branch(workdir),
        "submitted_by": args.submitted_by or os.getenv("USER", "unknown"),
        "submitted_at": now_iso(),
        "scoped_test_command": scoped_cmd or "",
        "status": "QUEUED",
        "steps": [],
        "artifacts": {},
    }

    with file_lock(DATA_LOCK, wait=True):
        pending = load_pending()
        pending.append(job)
        save_pending(pending)

    payload = {
        "job_id": job_id,
        "ticket_id": args.ticket,
        "mode": mode,
        "pending_file": str(PENDING_FILE),
        "history_file": str(history_path(job_id)),
    }
    if args.json:
        print(json.dumps(payload, indent=2, ensure_ascii=True))
    else:
        print(f"ENQUEUED job_id={job_id} ticket={args.ticket} mode={mode}")
        print(f"HISTORY {history_path(job_id)}")
    return 0


def pick_next_job() -> dict[str, Any] | None:
    with file_lock(DATA_LOCK, wait=True):
        pending = load_pending()
        if not pending:
            return None
        job = pending.pop(0)
        save_pending(pending)
        return job


def write_history(job: dict[str, Any]) -> None:
    path = history_path(job["job_id"])
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(job, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")


def parse_kv_meta(path: Path) -> dict[str, str]:
    out: dict[str, str] = {}
    if not path.exists():
        return out
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        out[key.strip()] = value.strip()
    return out


def step_gate(job: dict[str, Any], step_name: str, args_list: list[str]) -> dict[str, Any]:
    global CURRENT_CHILD, RECEIVED_SIGNAL
    step_run_id = f"{datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')}_{job['job_id']}_{step_name}"
    env = os.environ.copy()
    env["GATE_RUN_ID"] = step_run_id

    cmd = ["bash", str(CARGO_GATE_SCRIPT), *args_list]
    started = now_iso()
    proc = subprocess.Popen(cmd, cwd=job["workdir"], env=env, text=True)
    CURRENT_CHILD = proc
    interrupted = False
    try:
        exit_code = proc.wait()
    except KeyboardInterrupt:
        interrupted = True
        if proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait(timeout=10)
        exit_code = 130
    finally:
        CURRENT_CHILD = None
    finished = now_iso()

    meta_file = GATES_DIR / f"{step_run_id}.meta"
    log_file = GATES_DIR / f"{step_run_id}.log"
    meta = parse_kv_meta(meta_file)

    step = {
        "name": step_name,
        "command": " ".join(shlex.quote(x) for x in cmd),
        "started_at": started,
        "finished_at": finished,
        "status": "PASS" if exit_code == 0 else "FAIL",
        "interrupted": interrupted,
        "signal": RECEIVED_SIGNAL if interrupted else None,
        "exit_code": exit_code,
        "effective_exit_code": exit_code,
        "artifacts": {
            "run_id": step_run_id,
            "meta_file": str(meta_file),
            "log_file": str(log_file),
            "meta": meta,
        },
    }
    if interrupted:
        step["status"] = "INTERRUPTED"
        step["effective_exit_code"] = 130
        raise GateInterrupted(step)
    return step


def evaluate_baseline_waiver(test_step: dict[str, Any], job: dict[str, Any]) -> dict[str, Any]:
    evaluator_out = HISTORY_DIR / f"{job['job_id']}_baseline_eval.json"
    log_file = test_step["artifacts"]["log_file"]
    cmd = [
        "python3",
        str(EVAL_SCRIPT),
        "--log",
        str(log_file),
        "--baseline",
        str(BASELINE_FILE),
        "--out",
        str(evaluator_out),
    ]
    result = subprocess.run(cmd, check=False, capture_output=True, text=True)
    summary: dict[str, Any] = {
        "command": " ".join(shlex.quote(x) for x in cmd),
        "exit_code": result.returncode,
        "artifact": str(evaluator_out),
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }
    if evaluator_out.exists():
        try:
            summary["result"] = json.loads(evaluator_out.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            summary["result"] = {}
    return summary


def run_job(job: dict[str, Any]) -> dict[str, Any]:
    mode = job["mode"]
    job["status"] = "RUNNING"
    job["started_at"] = now_iso()
    job["steps"] = []
    job["artifacts"] = {"history_file": str(history_path(job["job_id"]))}

    if mode == MODE_TICKET:
        scoped_tokens = parse_scoped_test_command(job.get("scoped_test_command", "cargo-safe test --lib"))
        steps = [
            ("check", ["check"]),
            ("scoped_test", scoped_tokens),
            ("fmt", ["fmt", "--", "--check"]),
        ]
    elif mode == MODE_MERGE:
        steps = [
            ("check", ["check"]),
            ("test", ["test"]),
            ("fmt", ["fmt", "--", "--check"]),
        ]
    else:
        raise ValueError(f"unsupported job mode: {mode}")

    waived = False
    for step_name, step_args in steps:
        try:
            step = step_gate(job, step_name, step_args)
        except GateInterrupted as interrupted:
            job["steps"].append(interrupted.step)
            job["status"] = "INTERRUPTED"
            job["finished_at"] = now_iso()
            return job
        if mode == MODE_MERGE and step_name == "test" and step["exit_code"] != 0:
            waiver = evaluate_baseline_waiver(step, job)
            step["baseline_waiver"] = waiver
            only_known = bool(waiver.get("result", {}).get("only_known_failures"))
            if only_known:
                waived = True
                step["status"] = "WAIVED_BASELINE"
                step["effective_exit_code"] = 0
            else:
                step["status"] = "FAIL"
                step["effective_exit_code"] = step["exit_code"]

        job["steps"].append(step)
        if step["effective_exit_code"] != 0:
            job["status"] = "FAIL"
            job["finished_at"] = now_iso()
            return job

    job["finished_at"] = now_iso()
    if waived:
        job["status"] = "WAIVED_BASELINE"
    else:
        job["status"] = "PASS"
    return job


def run_next(args: argparse.Namespace) -> int:
    install_signal_handlers()
    ensure_dirs()
    try:
        with file_lock(RUNNER_LOCK, wait=args.wait_lock):
            job = pick_next_job()
            if not job:
                if args.json:
                    print(json.dumps({"status": "IDLE"}, indent=2, ensure_ascii=True))
                else:
                    print("IDLE: no pending gate jobs")
                return 0

            with file_lock(DATA_LOCK, wait=True):
                set_running(job)

            try:
                result = run_job(job)
            except KeyboardInterrupt:
                result = job
                result["status"] = "INTERRUPTED"
                result["finished_at"] = now_iso()
                result["error"] = "runner interrupted by signal"
            except Exception as exc:  # pragma: no cover - defensive
                result = job
                result["status"] = "FAIL"
                result["finished_at"] = now_iso()
                result["error"] = str(exc)
            finally:
                with file_lock(DATA_LOCK, wait=True):
                    clear_running()
                    write_history(result)

            payload = {
                "job_id": result["job_id"],
                "ticket_id": result.get("ticket_id", ""),
                "mode": result.get("mode", ""),
                "status": result.get("status", "UNKNOWN"),
                "history_file": str(history_path(result["job_id"])),
            }
            if args.json:
                print(json.dumps(payload, indent=2, ensure_ascii=True))
            else:
                print(
                    f"COMPLETED job_id={payload['job_id']} ticket={payload['ticket_id']} "
                    f"mode={payload['mode']} status={payload['status']}"
                )
                print(f"HISTORY {payload['history_file']}")
            return 0 if payload["status"] in {"PASS", "WAIVED_BASELINE"} else 1
    except RuntimeError as exc:
        if str(exc).startswith("lock-busy:"):
            print("BUSY: gate runner lock held by another process", file=sys.stderr)
            return 75
        raise


def read_history(job_id: str) -> dict[str, Any] | None:
    path = history_path(job_id)
    if not path.exists():
        return None
    return json.loads(path.read_text(encoding="utf-8"))


def run_loop(args: argparse.Namespace) -> int:
    ensure_dirs()
    exit_code = 0
    while True:
        rc = run_next(argparse.Namespace(wait_lock=args.wait_lock, json=args.json))
        if rc == 75:
            time.sleep(args.sleep)
            continue

        if args.until_job:
            hist = read_history(args.until_job)
            if hist and hist.get("status") in {"PASS", "WAIVED_BASELINE", "FAIL"}:
                return 0 if hist["status"] in {"PASS", "WAIVED_BASELINE"} else 1
            time.sleep(args.sleep)
            continue

        if rc != 0:
            exit_code = rc
        if rc == 0:
            # Stop on idle.
            pending = load_pending()
            if not pending:
                return exit_code
        time.sleep(args.sleep)


def status(args: argparse.Namespace) -> int:
    ensure_dirs()
    pending = load_pending()
    running = json.loads(RUNNING_FILE.read_text(encoding="utf-8") or "{}")
    latest = sorted(HISTORY_DIR.glob("*.json"), key=lambda p: p.stat().st_mtime, reverse=True)[:10]

    payload: dict[str, Any] = {
        "pending_count": len(pending),
        "running": running if running else {},
        "latest_history": [str(p) for p in latest],
    }

    if args.json:
        print(json.dumps(payload, indent=2, ensure_ascii=True))
    else:
        print(f"pending={payload['pending_count']}")
        if running and running.get("job_id"):
            print(
                f"running job_id={running.get('job_id')} ticket={running.get('ticket_id')} "
                f"mode={running.get('mode')} status={running.get('status')}"
            )
        else:
            print("running=none")
        if latest:
            print("latest history:")
            for p in latest:
                print(f"- {p}")
    return 0


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(description="Gate queue manager")
    sub = p.add_subparsers(dest="cmd", required=True)

    p_enqueue = sub.add_parser("enqueue", help="Enqueue a gate job")
    p_enqueue.add_argument("--ticket", required=True)
    p_enqueue.add_argument("--mode", required=True, choices=[MODE_TICKET, MODE_MERGE, "ticket", "merge"])
    p_enqueue.add_argument("--workdir", default=str(ROOT))
    p_enqueue.add_argument("--branch")
    p_enqueue.add_argument("--submitted-by")
    p_enqueue.add_argument("--scoped-test")
    p_enqueue.add_argument("--json", action="store_true")

    p_next = sub.add_parser("run-next", help="Run the next queued gate job")
    p_next.add_argument("--wait-lock", action="store_true", help="Wait for runner lock instead of failing busy")
    p_next.add_argument("--json", action="store_true")

    p_loop = sub.add_parser("run-loop", help="Continuously process queued jobs")
    p_loop.add_argument("--wait-lock", action="store_true", help="Wait for runner lock instead of failing busy")
    p_loop.add_argument("--sleep", type=float, default=2.0)
    p_loop.add_argument("--until-job", help="Continue until this job reaches terminal history state")
    p_loop.add_argument("--json", action="store_true")

    p_status = sub.add_parser("status", help="Show queue status")
    p_status.add_argument("--json", action="store_true")

    return p


def main() -> int:
    args = parser().parse_args()
    if args.cmd == "enqueue":
        args.mode = args.mode.upper()
        return enqueue_job(args)
    if args.cmd == "run-next":
        return run_next(args)
    if args.cmd == "run-loop":
        return run_loop(args)
    if args.cmd == "status":
        return status(args)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
