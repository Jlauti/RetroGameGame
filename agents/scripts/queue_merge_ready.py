#!/usr/bin/env python3
"""Refresh readiness and enqueue merge-ready tickets into gate_queue."""

from __future__ import annotations

import argparse
import json
import subprocess
from datetime import date, datetime, timezone
from pathlib import Path
from typing import Any


def parse_dt(raw: str | None) -> datetime:
    if not raw:
        return datetime.max.replace(tzinfo=timezone.utc)
    value = str(raw).strip()
    if not value:
        return datetime.max.replace(tzinfo=timezone.utc)
    if value.endswith("Z"):
        value = value[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(value)
        if parsed.tzinfo is None:
            parsed = parsed.replace(tzinfo=timezone.utc)
        return parsed
    except ValueError:
        return datetime.max.replace(tzinfo=timezone.utc)


def ordering_key(ticket: dict[str, Any]) -> tuple[int, datetime, str]:
    critical = 0 if str(ticket.get("critical_path", "NO")).upper() == "YES" else 1
    last_activity = parse_dt(ticket.get("last_activity_date"))
    ticket_id = str(ticket.get("ticket_id", ""))
    return (critical, last_activity, ticket_id)


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        rows.append(json.loads(line))
    return rows


def refresh_readiness(root: Path, day: str) -> Path:
    scripts = root / "agents" / "scripts"
    subprocess.run(
        ["python3", str(scripts / "reconcile_ticket_state.py"), "--root", str(root), "--date", day],
        check=True,
    )
    subprocess.run(
        ["python3", str(scripts / "build_release_board.py"), "--root", str(root), "--date", day],
        check=True,
    )
    snapshot = root / "agents" / "status" / "release" / f"readiness_snapshot_{day}.json"
    if not snapshot.exists():
        raise SystemExit(f"snapshot missing after refresh: {snapshot}")
    return snapshot


def latest_merge_history(history_dir: Path) -> dict[str, dict[str, Any]]:
    latest: dict[str, dict[str, Any]] = {}
    if not history_dir.exists():
        return latest
    for path in sorted(history_dir.glob("*.json")):
        try:
            payload = load_json(path)
        except Exception:
            continue
        if str(payload.get("mode", "")).upper() != "MERGE":
            continue
        ticket = str(payload.get("ticket_id", "")).strip()
        if not ticket:
            continue
        ts = parse_dt(payload.get("finished_at") or payload.get("submitted_at"))
        prev = latest.get(ticket)
        if prev is None or ts >= parse_dt(prev.get("finished_at") or prev.get("submitted_at")):
            latest[ticket] = payload
    return latest


def enqueue_merge_job(root: Path, ticket_id: str, workdir: Path) -> None:
    scripts = root / "agents" / "scripts"
    subprocess.run(
        [
            "python3",
            str(scripts / "gate_queue.py"),
            "enqueue",
            "--ticket",
            ticket_id,
            "--mode",
            "MERGE",
            "--workdir",
            str(workdir),
        ],
        check=True,
    )


def main() -> int:
    parser = argparse.ArgumentParser(description="Queue merge-ready tickets for gate_queue run-loop")
    parser.add_argument("--root", default="/home/jl/git/RetroGameGame")
    parser.add_argument("--date", default=date.today().isoformat())
    parser.add_argument("--workdir", default="")
    parser.add_argument("--snapshot", default="")
    parser.add_argument("--max", type=int, default=0, help="Queue at most N tickets (0 = no limit)")
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--skip-refresh", action="store_true")
    parser.add_argument(
        "--requeue-passed",
        action="store_true",
        help="Requeue even if latest merge gate status is PASS/WAIVED_BASELINE",
    )
    parser.add_argument(
        "--skip-failed",
        action="store_true",
        help="Do not requeue tickets whose latest merge gate status is FAIL/INTERRUPTED",
    )
    args = parser.parse_args()

    root = Path(args.root).resolve()
    workdir = Path(args.workdir).resolve() if args.workdir else root

    if args.snapshot:
        snapshot_path = Path(args.snapshot).resolve()
        if not snapshot_path.exists():
            raise SystemExit(f"snapshot not found: {snapshot_path}")
    elif args.skip_refresh:
        snapshot_path = root / "agents" / "status" / "release" / f"readiness_snapshot_{args.date}.json"
        if not snapshot_path.exists():
            raise SystemExit(
                f"snapshot not found for --skip-refresh: {snapshot_path} (run reconcile first)"
            )
    else:
        snapshot_path = refresh_readiness(root, args.date)

    payload = load_json(snapshot_path)
    tickets = list(payload.get("tickets", []))
    candidates = [
        t for t in tickets if str(t.get("effective_status", "")).upper() == "READY_FOR_MERGE"
    ]
    candidates.sort(key=ordering_key)

    queue_dir = root / "agents" / "status" / "gates" / "queue"
    pending = load_jsonl(queue_dir / "pending.jsonl")
    running = load_json(queue_dir / "running.json") if (queue_dir / "running.json").exists() else {}
    latest_merge = latest_merge_history(queue_dir / "history")

    pending_merge_tickets = {
        str(row.get("ticket_id", "")).strip()
        for row in pending
        if str(row.get("mode", "")).upper() == "MERGE"
    }
    running_ticket = ""
    if str(running.get("mode", "")).upper() == "MERGE":
        running_ticket = str(running.get("ticket_id", "")).strip()

    queued = 0
    skipped = 0
    for ticket in candidates:
        ticket_id = str(ticket.get("ticket_id", "")).strip()
        if not ticket_id:
            continue

        if ticket_id in pending_merge_tickets or ticket_id == running_ticket:
            print(f"skip {ticket_id}: already queued/running")
            skipped += 1
            continue

        previous = latest_merge.get(ticket_id, {})
        previous_status = str(previous.get("status", "")).upper()
        if previous_status in {"PASS", "WAIVED_BASELINE"} and not args.requeue_passed:
            print(f"skip {ticket_id}: latest merge gate already {previous_status}")
            skipped += 1
            continue
        if previous_status in {"FAIL", "INTERRUPTED"} and args.skip_failed:
            print(f"skip {ticket_id}: latest merge gate is {previous_status} and --skip-failed is set")
            skipped += 1
            continue

        if args.dry_run:
            print(f"dry-run queue {ticket_id}")
        else:
            enqueue_merge_job(root, ticket_id, workdir)
            print(f"queued {ticket_id}")
        queued += 1
        if args.max > 0 and queued >= args.max:
            break

    print(
        f"queue_merge_ready snapshot={snapshot_path} candidates={len(candidates)} queued={queued} skipped={skipped}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
