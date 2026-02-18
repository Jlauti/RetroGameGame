#!/usr/bin/env python3
"""Append durable review context to an agent's personal memory file."""

from __future__ import annotations

import argparse
from datetime import datetime
from pathlib import Path

AGENT_MAP = {
    "agent1": "aarne_tasapaino",
    "aarne_tasapaino": "aarne_tasapaino",
    "agent2": "pekka_kone",
    "pekka_kone": "pekka_kone",
    "agent3": "ilmari_maasto",
    "ilmari_maasto": "ilmari_maasto",
    "agent4": "aino_kuvitus",
    "aino_kuvitus": "aino_kuvitus",
    "agent5": "veikko_fiilis",
    "veikko_fiilis": "veikko_fiilis",
    "qa": "sanna_laatu",
    "sanna_laatu": "sanna_laatu",
}


def ensure_memory(path: Path, agent_key: str) -> None:
    if path.exists():
        return
    lines = [
        f"# {agent_key} Memory",
        "",
        "## Review Notes",
        "",
    ]
    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Append memory note for agent workspace")
    parser.add_argument("--agent", required=True, help="agent id or codename")
    parser.add_argument("--note", required=True, help="memory note text")
    parser.add_argument("--ticket", default="", help="optional ticket id")
    parser.add_argument("--title", default="Review Memory", help="short title")
    args = parser.parse_args()

    agent_key = args.agent.strip().lower()
    if agent_key not in AGENT_MAP:
        valid = ", ".join(sorted(set(AGENT_MAP.keys())))
        raise SystemExit(f"Unknown agent '{args.agent}'. Valid: {valid}")

    codename = AGENT_MAP[agent_key]
    memory_path = Path(f"/home/jl/git/RetroGameGame/agents/team/{codename}/memory.md")
    ensure_memory(memory_path, codename)

    ts = datetime.now().strftime("%Y-%m-%d %H:%M")
    ticket_text = f" | ticket={args.ticket}" if args.ticket else ""
    block = [
        f"### {args.title}",
        f"- Recorded: {ts}{ticket_text}",
        f"- Note: {args.note}",
        "",
    ]

    with memory_path.open("a", encoding="utf-8") as f:
        f.write("\n".join(block))

    print(f"Appended memory for {args.agent} -> {memory_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
