#!/usr/bin/env python3
"""Enforce one-ticket WIP per agent."""

from __future__ import annotations

import argparse
import sys
from collections import defaultdict
from pathlib import Path

from agent_io import markdown_files, parse_metadata


def main() -> int:
    parser = argparse.ArgumentParser(description="Check one-ticket WIP policy")
    parser.add_argument(
        "--backlog",
        default="/home/jl/git/RetroGameGame/agents/backlog",
        help="Backlog directory",
    )
    args = parser.parse_args()

    backlog = Path(args.backlog)
    if not backlog.exists():
        print(f"ERROR: backlog directory does not exist: {backlog}", file=sys.stderr)
        return 2

    in_progress = defaultdict(list)

    for path in markdown_files(backlog):
        if path.name.upper().startswith("TEMPLATE"):
            continue
        meta = parse_metadata(path.read_text(encoding="utf-8"))
        owner = meta.get("owner_agent", "").strip()
        status = meta.get("status", "").upper().strip()
        if owner and status == "IN_PROGRESS":
            in_progress[owner].append(path.name)

    violations = {k: v for k, v in in_progress.items() if len(v) > 1}
    if violations:
        print("One-ticket WIP policy violation detected:", file=sys.stderr)
        for owner, tickets in sorted(violations.items()):
            print(f"- {owner}: {', '.join(tickets)}", file=sys.stderr)
        return 1

    print("One-ticket WIP policy passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
