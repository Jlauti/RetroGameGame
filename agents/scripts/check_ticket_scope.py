#!/usr/bin/env python3
"""Check git diff scope against ticket allowed paths."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path, PurePosixPath

from agent_io import parse_section_bullets

ALWAYS_ALLOWED = (
    "agents/",
)


def norm_path(value: str) -> str:
    cleaned = value.strip().replace("\\", "/")
    if cleaned.startswith("./"):
        cleaned = cleaned[2:]
    return str(PurePosixPath(cleaned))


def in_allowed(path: str, prefixes: list[str]) -> bool:
    for prefix in prefixes:
        p = prefix.rstrip("/")
        if path == p or path.startswith(p + "/"):
            return True
    return False


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate changed files against ticket scope")
    parser.add_argument("--ticket", required=True, help="Path to ticket markdown")
    parser.add_argument("--base", default="main", help="Base ref for git diff")
    args = parser.parse_args()

    ticket = Path(args.ticket)
    if not ticket.exists():
        print(f"ERROR: ticket not found: {ticket}", file=sys.stderr)
        return 2

    text = ticket.read_text(encoding="utf-8")
    allowed_paths = [norm_path(p) for p in parse_section_bullets(text, "Allowed Paths") if p.strip()]
    if not allowed_paths:
        print("ERROR: ticket has no allowed paths", file=sys.stderr)
        return 2

    result = subprocess.run(
        ["git", "diff", "--name-only", f"{args.base}...HEAD"],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        print(result.stderr.strip(), file=sys.stderr)
        return result.returncode

    changed = [norm_path(line) for line in result.stdout.splitlines() if line.strip()]
    if not changed:
        print("ERROR: no changed files found in diff", file=sys.stderr)
        return 1

    effective_allowed = allowed_paths + [norm_path(p) for p in ALWAYS_ALLOWED]
    disallowed = [path for path in changed if not in_allowed(path, effective_allowed)]

    print("Changed files:")
    for path in changed:
        print(f"- {path}")

    if disallowed:
        print("\nDisallowed changes:", file=sys.stderr)
        for path in disallowed:
            print(f"- {path}", file=sys.stderr)
        return 1

    print("\nScope check passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
