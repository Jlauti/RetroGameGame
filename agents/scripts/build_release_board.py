#!/usr/bin/env python3
"""Build a markdown release board from readiness snapshot."""

from __future__ import annotations

import argparse
import json
from collections import Counter, defaultdict
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
        pass
    try:
        return datetime.strptime(value, "%Y-%m-%d").replace(tzinfo=timezone.utc)
    except ValueError:
        return datetime.max.replace(tzinfo=timezone.utc)


def latest_snapshot(release_dir: Path) -> Path | None:
    candidates = sorted(release_dir.glob("readiness_snapshot_*.json"))
    if not candidates:
        return None
    return max(candidates, key=lambda p: p.stat().st_mtime_ns)


def ordering_key(ticket: dict[str, Any]) -> tuple[int, datetime, str]:
    critical = 0 if str(ticket.get("critical_path", "NO")).upper() == "YES" else 1
    last_activity = parse_dt(ticket.get("last_activity_date"))
    ticket_id = str(ticket.get("ticket_id", ""))
    return (critical, last_activity, ticket_id)


def format_ticket_line(ticket: dict[str, Any]) -> str:
    return (
        f"- {ticket.get('ticket_id')}: owner={ticket.get('owner_agent')}, "
        f"effective={ticket.get('effective_status')}, declared={ticket.get('declared_status')}, "
        f"critical={ticket.get('critical_path')}, last_activity={ticket.get('last_activity_date')}"
    )


def build_board(snapshot_path: Path, out_path: Path) -> Path:
    payload = json.loads(snapshot_path.read_text(encoding="utf-8"))
    tickets = list(payload.get("tickets", []))
    counts = Counter(str(t.get("effective_status", "UNKNOWN")) for t in tickets)

    merge_candidates = [
        t
        for t in tickets
        if str(t.get("base_effective_status", "")) == "READY_FOR_MERGE"
        or str(t.get("effective_status", "")) == "READY_FOR_MERGE"
    ]
    merge_candidates.sort(key=ordering_key)

    qa_queue = [
        t
        for t in tickets
        if str(t.get("base_effective_status", "")) == "READY_FOR_QA"
        or str(t.get("effective_status", "")) == "READY_FOR_QA"
    ]
    qa_queue.sort(key=ordering_key)

    stale = [t for t in tickets if str(t.get("effective_status", "")) == "STALE_METADATA"]
    stale.sort(key=ordering_key)

    nudge_targets: dict[str, list[str]] = defaultdict(list)
    for ticket in tickets:
        ticket_id = str(ticket.get("ticket_id", ""))
        owner = str(ticket.get("owner_agent", "unassigned"))
        effective = str(ticket.get("effective_status", "UNKNOWN"))
        if effective in {"IN_PROGRESS", "BLOCKED"}:
            nudge_targets[owner].append(f"{ticket_id} ({effective})")
        if effective == "READY_FOR_QA":
            nudge_targets["qa"].append(f"{ticket_id} (READY_FOR_QA)")
        if effective == "READY_FOR_MERGE":
            nudge_targets["principal_engineer"].append(f"{ticket_id} (READY_FOR_MERGE)")
        if effective == "STALE_METADATA":
            nudge_targets[owner].append(f"{ticket_id} (STALE_METADATA)")
            nudge_targets["principal_engineer"].append(f"{ticket_id} (STALE_METADATA)")

    lines: list[str] = []
    lines.append(f"# Release Board ({date.today().isoformat()})")
    lines.append("")
    lines.append("## Source")
    lines.append("")
    lines.append(f"- snapshot: {snapshot_path}")
    lines.append(f"- generated_at: {datetime.now(timezone.utc).isoformat()}")
    lines.append("")
    lines.append("## Effective Status Counts")
    lines.append("")
    for key in sorted(counts.keys()):
        lines.append(f"- {key}: {counts[key]}")
    lines.append("")
    lines.append("## Ordered Merge Candidates")
    lines.append("")
    if merge_candidates:
        for t in merge_candidates:
            lines.append(format_ticket_line(t))
    else:
        lines.append("- none")
    lines.append("")
    lines.append("## Ordered QA Queue")
    lines.append("")
    if qa_queue:
        for t in qa_queue:
            lines.append(format_ticket_line(t))
    else:
        lines.append("- none")
    lines.append("")
    lines.append("## Stale Metadata Tickets")
    lines.append("")
    if stale:
        for t in stale:
            lines.append(format_ticket_line(t))
            conflicts = t.get("conflicts", [])
            if conflicts:
                for conflict in conflicts:
                    lines.append(f"- conflict: {conflict}")
    else:
        lines.append("- none")
    lines.append("")
    lines.append("## Nudge Targets")
    lines.append("")
    if nudge_targets:
        for agent in sorted(nudge_targets.keys()):
            lines.append(f"- {agent}: {', '.join(sorted(set(nudge_targets[agent])))}")
    else:
        lines.append("- none")
    lines.append("")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text("\n".join(lines), encoding="utf-8")
    return out_path


def main() -> int:
    parser = argparse.ArgumentParser(description="Build release board from readiness snapshot")
    parser.add_argument("--root", default="/home/jl/git/RetroGameGame", help="Repository root")
    parser.add_argument("--date", default=date.today().isoformat(), help="Board date (YYYY-MM-DD)")
    parser.add_argument("--snapshot", default="", help="Explicit snapshot path")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    release_dir = root / "agents" / "status" / "release"
    if args.snapshot:
        snapshot_path = Path(args.snapshot).resolve()
    else:
        date_match = release_dir / f"readiness_snapshot_{args.date}.json"
        snapshot_path = date_match if date_match.exists() else latest_snapshot(release_dir)  # type: ignore[assignment]
    if not snapshot_path or not snapshot_path.exists():
        raise SystemExit("No readiness snapshot found. Run reconcile_ticket_state.py first.")

    out_path = release_dir / f"release_board_{args.date}.md"
    out_path = build_board(snapshot_path, out_path)
    print(f"release_board={out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
