#!/usr/bin/env python3
"""Generate daily status digest from backlog/reports/qa/merge artifacts."""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from datetime import date
from pathlib import Path
import json
import re
from typing import Dict, List

from agent_io import markdown_files, parse_metadata

AGENTS = ["agent1", "agent2", "agent3", "agent4", "agent5", "qa"]
ACTIVE_STATUSES = {"IN_PROGRESS", "BLOCKED", "READY_FOR_QA"}


def normalize_agent(value: str) -> str:
    match = re.search(r"\b(agent[1-5]|qa)\b", value.lower())
    if match:
        return match.group(1)
    return value.strip().lower()


def report_date(meta: Dict[str, str]) -> str:
    return meta.get("report_date", "").strip() or meta.get("date", "").strip()


def load_backlog(path: Path) -> List[Dict[str, str]]:
    rows: List[Dict[str, str]] = []
    for file in markdown_files(path):
        upper_name = file.name.upper()
        if upper_name.startswith("TEMPLATE") or file.name == "README.md":
            continue
        meta = parse_metadata(file.read_text(encoding="utf-8"))
        if not meta.get("ticket_id"):
            continue
        meta["_file"] = file.name
        rows.append(meta)
    return rows


def load_reports(path: Path) -> List[Dict[str, str]]:
    rows: List[Dict[str, str]] = []
    for agent_dir in sorted(path.iterdir() if path.exists() else []):
        if not agent_dir.is_dir():
            continue
        for file in markdown_files(agent_dir):
            meta = parse_metadata(file.read_text(encoding="utf-8"))
            mtime_date = date.fromtimestamp(file.stat().st_mtime).isoformat()
            meta["_file"] = str(file.relative_to(path))
            meta["_agent_id"] = normalize_agent(meta.get("agent", "") or agent_dir.name)
            meta["_report_date"] = report_date(meta) or mtime_date
            meta["_mtime"] = str(file.stat().st_mtime_ns)
            rows.append(meta)
    return rows


def load_by_ticket(path: Path, key: str) -> Dict[str, Dict[str, str]]:
    out: Dict[str, Dict[str, str]] = {}
    for file in markdown_files(path):
        if file.name.upper().startswith("TEMPLATE"):
            continue
        meta = parse_metadata(file.read_text(encoding="utf-8"))
        ticket = meta.get("ticket_id", "").strip()
        if not ticket:
            continue
        if ticket not in out:
            out[ticket] = {}
        out[ticket].update(meta)
        out[ticket]["_file"] = file.name
        out[ticket]["_kind"] = key
    return out


def load_current_milestone(path: Path) -> Dict[str, str]:
    if not path.exists():
        return {}
    return parse_metadata(path.read_text(encoding="utf-8"))


def load_queue_history(path: Path) -> List[Dict[str, str]]:
    rows: List[Dict[str, str]] = []
    if not path.exists():
        return rows
    for file in sorted(path.glob("*.json")):
        if not file.is_file():
            continue
        try:
            payload = json.loads(file.read_text(encoding="utf-8"))
        except Exception:
            continue
        payload["_file"] = str(file)
        rows.append(payload)
    return rows


def latest_report_for_agent(reports: List[Dict[str, str]], agent: str) -> Dict[str, str] | None:
    candidates = [r for r in reports if r.get("_agent_id", "") == agent]
    if not candidates:
        return None
    return sorted(
        candidates,
        key=lambda r: (r.get("_report_date", ""), r.get("_mtime", ""), r.get("_file", "")),
    )[-1]


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate daily agent status digest")
    parser.add_argument(
        "--root",
        default="/home/jl/git/RetroGameGame",
        help="Repository root",
    )
    parser.add_argument(
        "--date",
        default=date.today().isoformat(),
        help="Report date (YYYY-MM-DD)",
    )
    args = parser.parse_args()

    root = Path(args.root)
    agents_root = root / "agents"
    backlog = load_backlog(agents_root / "backlog")
    reports = load_reports(agents_root / "reports")
    qa = load_by_ticket(agents_root / "qa", key="qa")
    merge = load_by_ticket(agents_root / "merge", key="merge")
    milestone = load_current_milestone(agents_root / "status" / "current_milestone.md")
    queue_history = load_queue_history(agents_root / "status" / "gates" / "queue" / "history")

    tickets_by_agent: Dict[str, List[Dict[str, str]]] = defaultdict(list)
    for ticket in backlog:
        owner = ticket.get("owner_agent", "unassigned")
        tickets_by_agent[owner].append(ticket)

    lane_counter = Counter(t.get("execution_lane", "LOCAL") for t in backlog)

    jules_failures = []
    for report in reports:
        lane = report.get("execution_lane", "").upper()
        health = report.get("session_health", "").upper()
        if lane == "JULES" and health in {"UNHEALTHY", "TAKEN_OVER"}:
            jules_failures.append(report)

    merge_candidates = []
    for ticket in backlog:
        ticket_id = ticket.get("ticket_id", "")
        if not ticket_id:
            continue
        qa_result = qa.get(ticket_id, {}).get("gate_result", "UNKNOWN")
        merge_decision = merge.get(ticket_id, {}).get("merge_decision", "UNSET")
        if qa_result.upper() == "PASS" and merge_decision.upper() != "MERGED":
            merge_candidates.append((ticket_id, qa_result, merge_decision))

    sla_breaches = []
    for agent in AGENTS:
        active = [
            t
            for t in tickets_by_agent.get(agent, [])
            if t.get("status", "").upper() in ACTIVE_STATUSES
        ]
        if not active:
            continue
        has_report_today = any(
            r for r in reports if r.get("_agent_id", "") == agent and r.get("_report_date", "") == args.date
        )
        if not has_report_today:
            ticket_ids = ", ".join(sorted(t.get("ticket_id", "") for t in active if t.get("ticket_id")))
            sla_breaches.append((agent, ticket_ids or "active ticket"))

    lines = []
    lines.append(f"# Daily Agent Status Digest ({args.date})")
    lines.append("")

    lines.append("## Milestone Status")
    lines.append("")
    if milestone:
        lines.append(f"- milestone_id: {milestone.get('milestone_id', 'unknown')}")
        lines.append(f"- name: {milestone.get('name', 'unknown')}")
        lines.append(f"- status: {milestone.get('status', 'unknown')}")
        lines.append(f"- owner: {milestone.get('owner', 'unknown')}")
        lines.append(f"- target_exit: {milestone.get('target_exit', 'unknown')}")
    else:
        lines.append("- none")
    lines.append("")

    lines.append("## Per-Agent Progress and Blockers")
    lines.append("")
    for agent in AGENTS:
        agent_tickets = tickets_by_agent.get(agent, [])
        if not agent_tickets:
            lines.append(f"- {agent}: no assigned tickets")
            continue
        status_counts = Counter(t.get("status", "UNKNOWN") for t in agent_tickets)
        summary = ", ".join(f"{k}={v}" for k, v in sorted(status_counts.items()))
        blockers = [t.get("ticket_id", "") for t in agent_tickets if t.get("status", "").upper() == "BLOCKED"]
        blocker_text = f"; blockers: {', '.join(blockers)}" if blockers else ""
        latest = latest_report_for_agent(reports, agent)
        latest_text = (
            f"; latest report: {latest.get('_report_date', 'n/a') or 'n/a'} ({latest.get('status', 'n/a')})"
            if latest
            else "; latest report: none"
        )
        lines.append(f"- {agent}: {summary}{blocker_text}{latest_text}")
    lines.append("")

    lines.append("## Merge Candidates and QA Status")
    lines.append("")
    if merge_candidates:
        for ticket_id, qa_result, merge_decision in sorted(merge_candidates):
            lines.append(f"- {ticket_id}: QA={qa_result}, merge_decision={merge_decision}")
    else:
        lines.append("- none")
    lines.append("")

    lines.append("## Lane Mix")
    lines.append("")
    lines.append(f"- LOCAL: {lane_counter.get('LOCAL', 0)}")
    lines.append(f"- JULES: {lane_counter.get('JULES', 0)}")
    lines.append("")

    lines.append("## Jules Experiment Failures / Takeovers")
    lines.append("")
    if jules_failures:
        for report in sorted(
            jules_failures,
            key=lambda r: (r.get("_report_date", ""), r.get("ticket_id", ""), r.get("_file", "")),
        ):
            lines.append(
                f"- {report.get('ticket_id', 'unknown')}: health={report.get('session_health', 'unknown')}, "
                f"trigger={report.get('takeover_trigger', 'n/a')}, report={report.get('_file', 'n/a')}"
            )
    else:
        lines.append("- none")
    lines.append("")

    baseline_waivers_today = []
    for job in queue_history:
        if str(job.get("status", "")).upper() != "WAIVED_BASELINE":
            continue
        finished_at = str(job.get("finished_at", ""))
        if finished_at.startswith(args.date):
            baseline_waivers_today.append(job)

    lines.append("## Baseline Waiver Usage")
    lines.append("")
    lines.append(f"- count_today: {len(baseline_waivers_today)}")
    for job in baseline_waivers_today:
        lines.append(
            f"- job_id={job.get('job_id', 'unknown')}, ticket={job.get('ticket_id', 'unknown')}, "
            f"artifact={job.get('_file', 'unknown')}"
        )
    lines.append("")

    lines.append("## SLA Breaches")
    lines.append("")
    if sla_breaches:
        for agent, ticket_ids in sla_breaches:
            lines.append(f"- {agent}: missing report for {ticket_ids}")
    else:
        lines.append("- none")
    lines.append("")

    out_dir = agents_root / "status" / "daily"
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / f"{args.date}.md"
    out_path.write_text("\n".join(lines), encoding="utf-8")

    print(f"Wrote daily digest: {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
