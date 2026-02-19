#!/usr/bin/env python3
"""Generate per-agent workspace folders with personal backlog and kickoff context."""

from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Dict, List

from agent_io import markdown_files, parse_metadata, parse_section_bullets


@dataclass(frozen=True)
class AgentProfile:
    agent_id: str
    finnish_name: str
    codename: str
    role: str
    default_prompt: str


PROFILES: List[AgentProfile] = [
    AgentProfile(
        agent_id="agent1",
        finnish_name="Aarne Tasapaino",
        codename="aarne_tasapaino",
        role="Lead Developer and System Balancer",
        default_prompt="agent1_start_prompt.md",
    ),
    AgentProfile(
        agent_id="agent2",
        finnish_name="Pekka Kone",
        codename="pekka_kone",
        role="Engine and Physics Architect",
        default_prompt="agent2_start_prompt.md",
    ),
    AgentProfile(
        agent_id="agent3",
        finnish_name="Ilmari Maasto",
        codename="ilmari_maasto",
        role="Procedural Generation and Topography Architect",
        default_prompt="agent3_start_prompt.md",
    ),
    AgentProfile(
        agent_id="agent4",
        finnish_name="Aino Kuvitus",
        codename="aino_kuvitus",
        role="Art Direction and Asset Consistency Lead",
        default_prompt="agent4_start_prompt.md",
    ),
    AgentProfile(
        agent_id="agent5",
        finnish_name="Veikko Fiilis",
        codename="veikko_fiilis",
        role="Game Feel and UX Engineer",
        default_prompt="agent5_start_prompt.md",
    ),
    AgentProfile(
        agent_id="qa",
        finnish_name="Sanna Laatu",
        codename="sanna_laatu",
        role="QA Gatekeeper",
        default_prompt="qa_start_prompt.md",
    ),
]

STATUS_ORDER = {
    "IN_PROGRESS": 0,
    "BLOCKED": 1,
    "READY_FOR_QA": 2,
    "TODO": 3,
    "DONE": 4,
    "READY_TO_MERGE": 5,
    "MERGED": 6,
}


def report_date(meta: Dict[str, str]) -> str:
    return meta.get("report_date", "").strip() or meta.get("date", "").strip()


def load_backlog(backlog_dir: Path) -> List[Dict[str, str]]:
    rows: List[Dict[str, str]] = []
    for file in markdown_files(backlog_dir):
        if file.name == "README.md":
            continue
        meta = parse_metadata(file.read_text(encoding="utf-8"))
        if not meta.get("ticket_id"):
            continue
        meta["_file"] = file.name
        rows.append(meta)
    return rows


def find_prompt_for_ticket(prompts_dir: Path, agent_id: str, ticket_id: str) -> str | None:
    suffix = ticket_id.lower()
    candidates = [
        p.name
        for p in prompts_dir.glob(f"{agent_id}_kickoff_*.md")
        if suffix in p.name.lower()
    ]
    if not candidates:
        return None
    return sorted(candidates)[0]


def read_text(path: Path) -> str:
    if not path.exists():
        return ""
    return path.read_text(encoding="utf-8")


def extract_objective(ticket_text: str) -> str:
    if "## Objective" not in ticket_text:
        return "Objective not found."
    return ticket_text.split("## Objective", 1)[1].split("##", 1)[0].strip()


def parse_next_assignments(path: Path) -> Dict[str, str]:
    if not path.exists():
        return {}

    lines = path.read_text(encoding="utf-8").splitlines()
    rows = [line for line in lines if line.strip().startswith("|")]
    if len(rows) < 3:
        return {}

    def parse_row(row: str) -> list[str]:
        return [cell.strip() for cell in row.strip().strip("|").split("|")]

    header = parse_row(rows[0])
    idx_agent = header.index("Agent ID") if "Agent ID" in header else -1
    idx_next = header.index("Next Ticket") if "Next Ticket" in header else -1
    if idx_agent < 0 or idx_next < 0:
        return {}

    out: Dict[str, str] = {}
    for row in rows[2:]:
        cells = parse_row(row)
        if len(cells) <= max(idx_agent, idx_next):
            continue
        agent_id = cells[idx_agent].strip()
        next_ticket_raw = cells[idx_next].strip()
        if not agent_id:
            continue
        if next_ticket_raw.upper().startswith("NONE"):
            out[agent_id] = "__NONE__"
            continue
        # Allow values like "NB-CX-011 (merge-prep closeout)".
        next_ticket = next_ticket_raw.split(" ", 1)[0].strip()
        out[agent_id] = next_ticket
    return out


def latest_report_for_agent(reports_root: Path, agent_id: str) -> tuple[Path | None, Dict[str, str]]:
    report_dir = reports_root / agent_id
    if not report_dir.exists():
        return None, {}

    candidates = [p for p in report_dir.glob("*.md") if p.name != ".gitkeep"]
    if not candidates:
        return None, {}

    latest = max(candidates, key=lambda p: p.stat().st_mtime)
    return latest, parse_metadata(read_text(latest))


def ensure_memory_file(memory_path: Path, profile: AgentProfile) -> None:
    if memory_path.exists():
        return

    now = datetime.now().strftime("%Y-%m-%d %H:%M")
    lines = [
        f"# {profile.finnish_name} Memory",
        "",
        "Persistent context and reviewed lessons for this agent.",
        "",
        f"- Agent ID: {profile.agent_id}",
        f"- Created: {now}",
        "",
        "## Working Preferences",
        "",
        "- Keep this section updated with stable execution preferences.",
        "",
        "## Repeated Pitfalls",
        "",
        "- Add recurring mistakes and how to avoid them.",
        "",
        "## Proven Patterns",
        "",
        "- Add approaches that consistently worked well.",
        "",
        "## Review Notes",
        "",
        "- Append short dated notes after each major review cycle.",
        "",
    ]
    memory_path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    repo_root = Path("/home/jl/git/RetroGameGame")
    agents_root = repo_root / "agents"
    backlog_dir = agents_root / "backlog"
    delegation_root = agents_root / "delegations"
    reports_root = agents_root / "reports"
    deliverables_root = agents_root / "deliverables"
    prompts_dir = agents_root / "prompts"
    team_root = agents_root / "team"
    next_assignments = parse_next_assignments(agents_root / "status" / "next_assignments.md")

    tickets = load_backlog(backlog_dir)
    tickets_by_id = {t.get("ticket_id", ""): t for t in tickets if t.get("ticket_id")}

    roster_lines = [
        "# Agent Team Roster",
        "",
        "Per-agent personal workspaces for local Antigravity/Gemini runs.",
        "",
        "## Directory",
        "",
    ]

    for profile in PROFILES:
        home = team_root / profile.codename
        inbox = home / "inbox"
        home.mkdir(parents=True, exist_ok=True)
        inbox.mkdir(parents=True, exist_ok=True)

        owned = [t for t in tickets if t.get("owner_agent") == profile.agent_id]
        owned.sort(key=lambda t: (STATUS_ORDER.get(t.get("status", "TODO"), 99), t.get("ticket_id", "")))

        assigned_marker = next_assignments.get(profile.agent_id)
        assigned_none = assigned_marker == "__NONE__"
        assigned_ticket_id = assigned_marker.strip() if assigned_marker and not assigned_none else ""
        assigned_ticket = tickets_by_id.get(assigned_ticket_id) if assigned_ticket_id else None

        roster_lines.append(
            f"- {profile.finnish_name} (`{profile.agent_id}`): "
            f"`/home/jl/git/RetroGameGame/agents/team/{profile.codename}`"
        )

        readme_lines = [
            f"# {profile.finnish_name}",
            "",
            f"- Agent ID: {profile.agent_id}",
            f"- Role: {profile.role}",
            f"- Workspace: /home/jl/git/RetroGameGame/agents/team/{profile.codename}",
            "",
            "## Start Here",
            "",
            "1. Read `nudge.md` (single-document identity + task packet).",
            "2. Use `launch_prompt.md` if you need full role constraints.",
            "3. Write reports to the canonical path under `/agents/reports/...`.",
            "4. If blocked, report with concrete unblock request.",
        ]
        (home / "README.md").write_text("\n".join(readme_lines) + "\n", encoding="utf-8")

        backlog_lines = [
            f"# {profile.finnish_name} Backlog",
            "",
            "| Ticket | Status | Execution Lane | Work Category |",
            "|---|---|---|---|",
        ]
        if owned:
            for t in owned:
                backlog_lines.append(
                    f"| {t.get('ticket_id', '?')} | {t.get('status', '?')} | "
                    f"{t.get('execution_lane', '?')} | {t.get('work_category', '?')} |"
                )
        else:
            backlog_lines.append("| none | none | none | none |")
        (home / "backlog.md").write_text("\n".join(backlog_lines) + "\n", encoding="utf-8")

        for old in inbox.glob("*.md"):
            old.unlink()

        if assigned_none:
            active_meta = None
        else:
            active_meta = assigned_ticket if assigned_ticket else (owned[0] if owned else None)

        launch_prompt_name = profile.default_prompt
        if active_meta:
            active_ticket = active_meta.get("ticket_id", "")
            kickoff = find_prompt_for_ticket(prompts_dir, profile.agent_id, active_ticket)
            if kickoff:
                launch_prompt_name = kickoff

        if owned:
            for t in owned:
                ticket_id = t.get("ticket_id", "")
                if not ticket_id:
                    continue
                ticket_path = backlog_dir / f"{ticket_id}.md"
                delegation_path = delegation_root / profile.agent_id / f"{ticket_id}_task.md"
                report_path = reports_root / profile.agent_id / f"{ticket_id}_task_report.md"
                ticket_text = read_text(ticket_path)

                card = [
                    f"# {ticket_id}",
                    "",
                    f"- Status: {t.get('status', 'UNKNOWN')}",
                    f"- Execution Lane: {t.get('execution_lane', 'LOCAL')}",
                    f"- Critical Path: {t.get('critical_path', 'NO')}",
                    "",
                    "## Canonical Files",
                    "",
                    f"- Ticket: `{ticket_path}`",
                    f"- Delegation: `{delegation_path}`",
                    f"- Report Target: `{report_path}`",
                    "",
                    "## Quick Objective",
                    "",
                    ticket_text.split("## Objective", 1)[1].split("##", 1)[0].strip()
                    if "## Objective" in ticket_text
                    else "Objective not found.",
                ]
                (inbox / f"{ticket_id}.md").write_text("\n".join(card) + "\n", encoding="utf-8")

        launch_path = prompts_dir / launch_prompt_name
        launch_text = read_text(launch_path)
        if not launch_text:
            launch_text = (
                "Launch prompt file not found. Use the canonical prompt README:\n"
                "/home/jl/git/RetroGameGame/agents/prompts/README.md\n"
            )
        (home / "launch_prompt.md").write_text(launch_text, encoding="utf-8")

        active_ticket_meta = active_meta
        nudge_lines = [
            f"# {profile.finnish_name} - Nudge Packet",
            "",
            "Read this file only. It contains who you are and what to do next.",
            "",
            "## Identity",
            "",
            f"- Agent ID: {profile.agent_id}",
            f"- Name: {profile.finnish_name}",
            f"- Role: {profile.role}",
            f"- Workspace Anchor: /home/jl/git/RetroGameGame/agents/team/{profile.codename}",
            f"- Launch Prompt: /home/jl/git/RetroGameGame/agents/prompts/{launch_prompt_name}",
            "",
            "## Current Task",
            "",
        ]

        if active_ticket_meta:
            ticket_id = active_ticket_meta.get("ticket_id", "unknown")
            ticket_path = backlog_dir / f"{ticket_id}.md"
            delegation_path = delegation_root / profile.agent_id / f"{ticket_id}_task.md"
            report_path = reports_root / profile.agent_id / f"{ticket_id}_task_report.md"
            ticket_text = read_text(ticket_path)
            objective = extract_objective(ticket_text)
            acceptance_commands = parse_section_bullets(ticket_text, "Acceptance Commands")
            scoped_test = active_ticket_meta.get("scoped_test_command", "").strip()

            nudge_lines.extend(
                [
                    f"- Ticket: {ticket_id}",
                    f"- Status: {active_ticket_meta.get('status', 'UNKNOWN')}",
                    f"- Execution Lane: {active_ticket_meta.get('execution_lane', 'LOCAL')}",
                    f"- Critical Path: {active_ticket_meta.get('critical_path', 'NO')}",
                    "",
                    "### Canonical Files",
                    "",
                    f"- Ticket: `{ticket_path}`",
                    f"- Delegation: `{delegation_path}`",
                    f"- Report Target: `{report_path}`",
                    "",
                    "### Objective",
                    "",
                    objective,
                    "",
                ]
            )
            if scoped_test:
                nudge_lines.extend(
                    [
                        "### Scoped Test Command",
                        "",
                        f"- `{scoped_test}`",
                        "",
                    ]
                )
            if acceptance_commands:
                nudge_lines.extend(
                    [
                        "### Acceptance Commands",
                        "",
                    ]
                )
                for cmd in acceptance_commands:
                    nudge_lines.append(f"- `{cmd}`")
                nudge_lines.append("")
            nudge_lines.extend(
                [
                    "## Action",
                    "",
                    "Execute the current task and write the report to the canonical report target.",
                    "",
                ]
            )
        else:
            nudge_lines.extend(
                [
                    "- No active ticket assigned.",
                    "",
                    "## Action",
                    "",
                    "Wait for principal engineer assignment. Keep this folder open as your identity anchor.",
                    "",
                ]
            )
        (home / "nudge.md").write_text("\n".join(nudge_lines), encoding="utf-8")

        ensure_memory_file(home / "memory.md", profile)

        latest_report_path, latest_report_meta = latest_report_for_agent(reports_root, profile.agent_id)
        deliverables_dir = deliverables_root / profile.agent_id
        deliverables = (
            sorted(
                [p.name for p in deliverables_dir.glob("*") if p.is_file() and not p.name.startswith(".")]
            )
            if deliverables_dir.exists()
            else []
        )
        active_ticket = active_meta.get("ticket_id", "none") if active_meta else "none"
        latest_report_status = latest_report_meta.get("status", "none")
        latest_report_date = report_date(latest_report_meta) or "n/a"

        context_lines = [
            f"# {profile.finnish_name} Context",
            "",
            f"- Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}",
            f"- Current Priority Ticket: {active_ticket}",
            f"- Latest Report Status: {latest_report_status}",
            f"- Latest Report Date: {latest_report_date}",
            f"- Latest Report Path: {latest_report_path if latest_report_path else 'none'}",
            "",
            "## Active Tickets",
            "",
        ]
        if owned:
            for t in owned:
                context_lines.append(
                    f"- {t.get('ticket_id', '?')}: status={t.get('status', '?')}, lane={t.get('execution_lane', '?')}"
                )
        else:
            context_lines.append("- none")

        context_lines.extend(
            [
                "",
                "## Recent Deliverables",
                "",
            ]
        )
        if deliverables:
            for name in deliverables[-10:]:
                context_lines.append(f"- {name}")
        else:
            context_lines.append("- none")

        context_lines.extend(
            [
                "",
                "## Memory Reminder",
                "",
                "- Read `memory.md` before starting work.",
                "- After review, principal engineer should append durable lessons to `memory.md`.",
                "",
            ]
        )
        (home / "context.md").write_text("\n".join(context_lines), encoding="utf-8")

    (agents_root / "team_roster.md").write_text("\n".join(roster_lines) + "\n", encoding="utf-8")
    print("Generated per-agent workspaces and roster.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
