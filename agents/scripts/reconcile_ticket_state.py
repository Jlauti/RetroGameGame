#!/usr/bin/env python3
"""Reconcile backlog, report, QA, merge, and gate artifacts into readiness snapshot."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
from dataclasses import dataclass
from datetime import date, datetime, timezone
from pathlib import Path
from typing import Any

from agent_io import markdown_files, parse_metadata

PASS_STATUSES = {"PASS", "DONE", "COMPLETED", "SUCCESS", "WAIVED_BASELINE"}
BLOCKED_STATUSES = {"FAIL", "FAILED", "BLOCKED", "ERROR", "INTERRUPTED", "UNHEALTHY"}
DECLARED_ACTIVE = {"TODO", "OPEN", "NEW", "IN_PROGRESS"}
DECLARED_QA = {"READY_FOR_QA"}
DECLARED_MERGE = {"READY_TO_MERGE", "READY_FOR_MERGE"}


@dataclass
class Artifact:
    data: dict[str, Any]
    path: Path
    timestamp: datetime


def parse_dt(raw: str | None) -> datetime | None:
    if not raw:
        return None
    value = str(raw).strip()
    if not value:
        return None
    if value.endswith("Z"):
        value = value[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(value)
        if parsed.tzinfo is None:
            parsed = parsed.replace(tzinfo=timezone.utc)
        return parsed
    except ValueError:
        pass
    for fmt in ("%Y-%m-%d", "%Y/%m/%d"):
        try:
            dt = datetime.strptime(value, fmt)
            return dt.replace(tzinfo=timezone.utc)
        except ValueError:
            continue
    return None


def to_status(raw: str | None) -> str:
    return (raw or "").strip().upper()


def is_yes(raw: str | None) -> bool:
    return (raw or "").strip().upper().startswith("YES")


def is_pass(raw: str | None) -> bool:
    return to_status(raw) in PASS_STATUSES


def is_blocked(raw: str | None) -> bool:
    return to_status(raw) in BLOCKED_STATUSES


def artifact_timestamp(meta: dict[str, Any], fallback_epoch_ns: int) -> datetime:
    for key in ("finished_at", "submitted_at", "report_date", "date"):
        parsed = parse_dt(meta.get(key))
        if parsed:
            return parsed
    return datetime.fromtimestamp(fallback_epoch_ns / 1_000_000_000, tz=timezone.utc)


def load_backlog(backlog_dir: Path) -> dict[str, Artifact]:
    out: dict[str, Artifact] = {}
    for file in markdown_files(backlog_dir):
        if file.name == "README.md":
            continue
        text = file.read_text(encoding="utf-8")
        meta = parse_metadata(text)
        ticket_id = (meta.get("ticket_id") or "").strip()
        if not ticket_id:
            continue
        ts = artifact_timestamp(meta, file.stat().st_mtime_ns)
        out[ticket_id] = Artifact(meta, file.resolve(), ts)
    return out


def load_reports(reports_root: Path) -> dict[str, Artifact]:
    latest: dict[str, Artifact] = {}
    if not reports_root.exists():
        return latest
    for agent_dir in sorted(reports_root.iterdir()):
        if not agent_dir.is_dir():
            continue
        for file in markdown_files(agent_dir):
            meta = parse_metadata(file.read_text(encoding="utf-8"))
            ticket_id = (meta.get("ticket_id") or "").strip()
            if not ticket_id:
                continue
            meta = dict(meta)
            meta.setdefault("agent", agent_dir.name)
            ts = artifact_timestamp(meta, file.stat().st_mtime_ns)
            current = latest.get(ticket_id)
            if current is None or ts >= current.timestamp:
                latest[ticket_id] = Artifact(meta, file.resolve(), ts)
    return latest


def load_qa(qa_dir: Path) -> dict[str, Artifact]:
    latest: dict[str, Artifact] = {}
    for file in markdown_files(qa_dir):
        if file.name in {"README.md", "checklist_template.md", "signoff_protocol.md"}:
            continue
        meta = parse_metadata(file.read_text(encoding="utf-8"))
        ticket_id = (meta.get("ticket_id") or "").strip()
        if not ticket_id:
            continue
        ts = artifact_timestamp(meta, file.stat().st_mtime_ns)
        current = latest.get(ticket_id)
        if current is None or ts >= current.timestamp:
            latest[ticket_id] = Artifact(meta, file.resolve(), ts)
    return latest


def load_merge(merge_dir: Path) -> dict[str, Artifact]:
    latest: dict[str, Artifact] = {}
    for file in markdown_files(merge_dir):
        if file.name == "README.md" or file.name.startswith("merge_order_"):
            continue
        meta = parse_metadata(file.read_text(encoding="utf-8"))
        ticket_id = (meta.get("ticket_id") or "").strip()
        if not ticket_id:
            continue
        ts = artifact_timestamp(meta, file.stat().st_mtime_ns)
        current = latest.get(ticket_id)
        if current is None or ts >= current.timestamp:
            latest[ticket_id] = Artifact(meta, file.resolve(), ts)
    return latest


def load_gate_history(history_dir: Path) -> tuple[dict[str, Artifact], dict[str, Artifact]]:
    latest_all: dict[str, Artifact] = {}
    latest_ticket_mode: dict[str, Artifact] = {}
    if not history_dir.exists():
        return latest_all, latest_ticket_mode
    for file in sorted(history_dir.glob("*.json")):
        if not file.is_file():
            continue
        try:
            payload = json.loads(file.read_text(encoding="utf-8"))
        except Exception:
            continue
        ticket_id = str(payload.get("ticket_id", "")).strip()
        if not ticket_id:
            continue
        ts = artifact_timestamp(payload, file.stat().st_mtime_ns)
        artifact = Artifact(payload, file.resolve(), ts)
        prev = latest_all.get(ticket_id)
        if prev is None or ts >= prev.timestamp:
            latest_all[ticket_id] = artifact
        if str(payload.get("mode", "")).upper() == "TICKET":
            prev_ticket = latest_ticket_mode.get(ticket_id)
            if prev_ticket is None or ts >= prev_ticket.timestamp:
                latest_ticket_mode[ticket_id] = artifact
    return latest_all, latest_ticket_mode


def choose_base_effective(
    declared_status: str,
    report_status: str,
    report_pass: bool,
    report_blocked: bool,
    qa_status: str,
    qa_pass: bool,
    qa_fail: bool,
    merged: bool,
    manifest_ready: bool,
    ticket_gate_status: str,
    ticket_gate_pass: bool,
    ticket_gate_blocked: bool,
    malformed: bool,
) -> str:
    if malformed:
        return "BLOCKED"
    if merged:
        return "MERGED"
    if report_blocked or qa_fail or ticket_gate_blocked:
        return "BLOCKED"
    if qa_pass and ticket_gate_pass and manifest_ready:
        return "READY_FOR_MERGE"
    if report_pass and not qa_pass:
        return "READY_FOR_QA"
    if declared_status in DECLARED_QA:
        return "READY_FOR_QA"
    if declared_status == "BLOCKED":
        return "BLOCKED"
    return "IN_PROGRESS"


def stale_conflicts(
    declared_status: str,
    base_effective: str,
    report_pass: bool,
    report_blocked: bool,
    qa_pass: bool,
    qa_fail: bool,
    ticket_gate_pass: bool,
    ticket_gate_blocked: bool,
) -> list[str]:
    conflicts: list[str] = []
    if report_pass and (qa_fail or ticket_gate_blocked):
        conflicts.append("report indicates PASS while QA/gate indicates BLOCKED")
    if qa_pass and ticket_gate_blocked:
        conflicts.append("QA PASS conflicts with latest ticket gate BLOCKED")

    # Declared status ahead of evidence is stale metadata.
    if declared_status in DECLARED_MERGE and base_effective in {"IN_PROGRESS", "READY_FOR_QA"}:
        conflicts.append(
            f"declared_status={declared_status} is ahead of evidence_state={base_effective}"
        )
    if declared_status == "MERGED" and base_effective != "MERGED":
        conflicts.append("declared_status=MERGED but merge evidence is missing")

    # Blocked declaration that has only pass evidence is stale metadata.
    if declared_status == "BLOCKED" and base_effective in {"READY_FOR_QA", "READY_FOR_MERGE", "MERGED"}:
        conflicts.append(
            f"declared_status=BLOCKED conflicts with evidence_state={base_effective}"
        )
    return conflicts


def reconcile(args: argparse.Namespace) -> tuple[Path, list[dict[str, Any]]]:
    root = Path(args.root).resolve()
    agents_root = root / "agents"
    backlog = load_backlog(agents_root / "backlog")
    reports = load_reports(agents_root / "reports")
    qa = load_qa(agents_root / "qa")
    merge = load_merge(agents_root / "merge")
    gates_all, gates_ticket = load_gate_history(agents_root / "status" / "gates" / "queue" / "history")

    rows: list[dict[str, Any]] = []
    for ticket_id, artifact in sorted(backlog.items()):
        meta = artifact.data
        declared_status = to_status(meta.get("status"))
        owner_agent = (meta.get("owner_agent") or "").strip() or "unassigned"
        critical = "YES" if to_status(meta.get("critical_path")) == "YES" else "NO"
        lane = (meta.get("execution_lane") or "LOCAL").strip().upper()

        evidence: list[str] = [f"declared_status={declared_status or 'UNKNOWN'}"]
        conflicts: list[str] = []

        required_keys = ("status", "owner_agent", "execution_lane", "critical_path")
        missing_keys = [k for k in required_keys if not str(meta.get(k, "")).strip()]
        malformed = bool(missing_keys)
        if malformed:
            evidence.append(f"metadata_missing={','.join(missing_keys)}")

        report_art = reports.get(ticket_id)
        report_status = ""
        report_path: str | None = None
        report_ts: datetime | None = None
        report_pass = False
        report_blocked = False
        if report_art:
            report_status = to_status(report_art.data.get("status"))
            report_path = str(report_art.path)
            report_ts = report_art.timestamp
            report_pass = is_pass(report_status)
            report_blocked = is_blocked(report_status)
            evidence.append(f"report_status={report_status or 'UNKNOWN'}")

        qa_art = qa.get(ticket_id)
        qa_status = ""
        qa_path: str | None = None
        qa_ts: datetime | None = None
        qa_pass = False
        qa_fail = False
        if qa_art:
            qa_status = to_status(qa_art.data.get("gate_result"))
            qa_path = str(qa_art.path)
            qa_ts = qa_art.timestamp
            qa_pass = qa_status == "PASS"
            qa_fail = is_blocked(qa_status)
            evidence.append(f"qa_result={qa_status or 'UNKNOWN'}")

        merge_art = merge.get(ticket_id)
        merge_path: str | None = None
        merge_ts: datetime | None = None
        merged = False
        manifest_ready = False
        manifest_gate_status = ""
        if merge_art:
            merge_meta = merge_art.data
            merge_path = str(merge_art.path)
            merge_ts = merge_art.timestamp
            merge_decision = to_status(merge_meta.get("merge_decision"))
            merged = merge_decision == "MERGED"
            manifest_gate_status = to_status(merge_meta.get("gate_status"))
            manifest_ready = (
                is_yes(merge_meta.get("ready_for_merge_to_main_now"))
                or is_yes(merge_meta.get("ready_for_pr_creation"))
                or is_pass(manifest_gate_status)
            )
            evidence.append(f"merge_manifest_present=YES")
            if merge_decision:
                evidence.append(f"merge_decision={merge_decision}")

        gate_art = gates_all.get(ticket_id)
        ticket_gate_art = gates_ticket.get(ticket_id)
        latest_gate_job_id = ""
        ticket_gate_status = ""
        ticket_gate_pass = False
        ticket_gate_blocked = False
        gate_ts: datetime | None = None
        if gate_art:
            latest_gate_job_id = str(gate_art.data.get("job_id", "")).strip()
            gate_ts = gate_art.timestamp
        if ticket_gate_art:
            ticket_gate_status = to_status(ticket_gate_art.data.get("status"))
            ticket_gate_pass = is_pass(ticket_gate_status)
            ticket_gate_blocked = is_blocked(ticket_gate_status)
            evidence.append(f"ticket_gate_status={ticket_gate_status or 'UNKNOWN'}")
        elif manifest_gate_status:
            ticket_gate_status = manifest_gate_status
            ticket_gate_pass = is_pass(ticket_gate_status)
            ticket_gate_blocked = is_blocked(ticket_gate_status)
            evidence.append(f"manifest_gate_status={ticket_gate_status}")

        base_effective = choose_base_effective(
            declared_status=declared_status,
            report_status=report_status,
            report_pass=report_pass,
            report_blocked=report_blocked,
            qa_status=qa_status,
            qa_pass=qa_pass,
            qa_fail=qa_fail,
            merged=merged,
            manifest_ready=manifest_ready,
            ticket_gate_status=ticket_gate_status,
            ticket_gate_pass=ticket_gate_pass,
            ticket_gate_blocked=ticket_gate_blocked,
            malformed=malformed,
        )
        conflicts.extend(
            stale_conflicts(
                declared_status=declared_status,
                base_effective=base_effective,
                report_pass=report_pass,
                report_blocked=report_blocked,
                qa_pass=qa_pass,
                qa_fail=qa_fail,
                ticket_gate_pass=ticket_gate_pass,
                ticket_gate_blocked=ticket_gate_blocked,
            )
        )

        effective = "STALE_METADATA" if conflicts else base_effective
        if conflicts:
            evidence.append(f"conflict_count={len(conflicts)}")

        dates = [d for d in (artifact.timestamp, report_ts, qa_ts, merge_ts, gate_ts) if d]
        last_activity = max(dates).astimezone(timezone.utc).isoformat() if dates else None

        row = {
            "ticket_id": ticket_id,
            "owner_agent": owner_agent,
            "critical_path": critical,
            "declared_status": declared_status or "UNKNOWN",
            "effective_status": effective,
            "base_effective_status": base_effective,
            "lane": lane,
            "latest_report_path": report_path,
            "qa_signoff_path": qa_path,
            "merge_manifest_path": merge_path,
            "latest_gate_job_id": latest_gate_job_id or None,
            "evidence": evidence,
            "conflicts": conflicts,
            "last_activity_date": last_activity,
        }
        rows.append(row)

    out_dir = agents_root / "status" / "release"
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / f"readiness_snapshot_{args.date}.json"
    payload = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_root": str(root),
        "tickets": rows,
    }
    out_path.write_text(json.dumps(payload, indent=2, ensure_ascii=True) + "\n", encoding="utf-8")
    return out_path, rows


def main() -> int:
    parser = argparse.ArgumentParser(description="Reconcile ticket state into release readiness snapshot")
    parser.add_argument("--root", default="/home/jl/git/RetroGameGame", help="Repository root")
    parser.add_argument("--date", default=date.today().isoformat(), help="Snapshot date (YYYY-MM-DD)")
    args = parser.parse_args()

    out_path, rows = reconcile(args)
    counts = Counter(row.get("effective_status", "UNKNOWN") for row in rows)
    summary = " ".join(f"{k}={v}" for k, v in sorted(counts.items()))
    print(f"summary total={len(rows)} {summary} snapshot={out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
