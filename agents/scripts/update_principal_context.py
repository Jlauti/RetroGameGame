#!/usr/bin/env python3
"""Refresh principal engineer context snapshot for session handoffs."""

from __future__ import annotations

from datetime import datetime
from pathlib import Path

from agent_io import markdown_files, parse_metadata

ROOT = Path("/home/jl/git/RetroGameGame")
AGENTS = ROOT / "agents"


def latest_daily_digest() -> Path | None:
    daily_dir = AGENTS / "status" / "daily"
    files = [p for p in markdown_files(daily_dir) if p.name != ".gitkeep"]
    if not files:
        return None
    return max(files, key=lambda p: p.name)


def active_tickets() -> list[dict[str, str]]:
    backlog_dir = AGENTS / "backlog"
    rows: list[dict[str, str]] = []
    for f in markdown_files(backlog_dir):
        if f.name == "README.md":
            continue
        meta = parse_metadata(f.read_text(encoding="utf-8"))
        if not meta.get("ticket_id"):
            continue
        status = meta.get("status", "")
        if status in {"IN_PROGRESS", "BLOCKED", "READY_FOR_QA"}:
            rows.append(meta)
    rows.sort(key=lambda r: (r.get("owner_agent", ""), r.get("ticket_id", "")))
    return rows


def main() -> int:
    milestone_path = AGENTS / "status" / "current_milestone.md"
    milestone = parse_metadata(milestone_path.read_text(encoding="utf-8")) if milestone_path.exists() else {}
    daily = latest_daily_digest()
    active = active_tickets()

    lines = [
        "# Principal Engineer Current Context",
        "",
        f"- Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}",
        f"- Current Milestone: {milestone.get('milestone_id', 'unknown')} ({milestone.get('name', 'unknown')})",
        f"- Milestone Status: {milestone.get('status', 'unknown')}",
        f"- Milestone Owner: {milestone.get('owner', 'unknown')}",
        f"- Latest Daily Digest: {daily if daily else 'none'}",
        "",
        "## Active Tickets",
        "",
    ]

    if active:
        for t in active:
            lines.append(
                f"- {t.get('ticket_id', '?')} | owner={t.get('owner_agent', '?')} | "
                f"status={t.get('status', '?')} | lane={t.get('execution_lane', '?')}"
            )
    else:
        lines.append("- none")

    lines.extend(
        [
            "",
            "## Session Resume Checklist",
            "",
            "1. Read principal `memory.md`.",
            "2. Read latest daily digest.",
            "3. Confirm blocker owners and next dispatches.",
            "4. Regenerate agent workspaces if ticket status changed.",
            "",
            "## Useful Commands",
            "",
            "```bash",
            "python3 /home/jl/git/RetroGameGame/agents/scripts/sync_agent_workspaces.py",
            "python3 /home/jl/git/RetroGameGame/agents/scripts/generate_daily_status.py --root /home/jl/git/RetroGameGame",
            "python3 /home/jl/git/RetroGameGame/agents/scripts/check_wip.py --backlog /home/jl/git/RetroGameGame/agents/backlog",
            "```",
            "",
        ]
    )

    out = AGENTS / "principal_engineer" / "current_context.md"
    out.write_text("\n".join(lines), encoding="utf-8")
    print(f"Updated {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
