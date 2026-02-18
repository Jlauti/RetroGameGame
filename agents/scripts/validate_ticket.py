#!/usr/bin/env python3
"""Validate ticket metadata and lane policy constraints."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from agent_io import parse_metadata, parse_section_bullets

REQUIRED = [
    "ticket_id",
    "owner_agent",
    "status",
    "execution_lane",
    "critical_path",
    "jules_eligible",
    "fallback_owner",
]
RECOMMENDED = ["scoped_test_command"]

ALLOWED_LANES = {"LOCAL", "JULES"}
ALLOWED_BOOL = {"YES", "NO"}
ALLOWED_STATUS = {
    "TODO",
    "IN_PROGRESS",
    "WAITING_FOR_GATES",
    "BLOCKED",
    "READY_FOR_QA",
    "DONE",
    "READY_TO_MERGE",
    "MERGED",
}


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate an agent-loop ticket")
    parser.add_argument("--ticket", required=True, help="Path to ticket markdown")
    args = parser.parse_args()

    ticket_path = Path(args.ticket)
    if not ticket_path.exists():
        print(f"ERROR: ticket not found: {ticket_path}", file=sys.stderr)
        return 2

    text = ticket_path.read_text(encoding="utf-8")
    meta = parse_metadata(text)
    allowed_paths = parse_section_bullets(text, "Allowed Paths")

    errors = []

    for key in REQUIRED:
        if not meta.get(key):
            errors.append(f"Missing metadata key: {key}")

    warnings = []
    for key in RECOMMENDED:
        if not meta.get(key):
            warnings.append(f"Missing recommended metadata key: {key}")

    lane = meta.get("execution_lane", "").upper()
    critical = meta.get("critical_path", "").upper()
    eligible = meta.get("jules_eligible", "").upper()
    status = meta.get("status", "").upper()

    if lane and lane not in ALLOWED_LANES:
        errors.append(f"Invalid execution lane: {lane}")
    if critical and critical not in ALLOWED_BOOL:
        errors.append(f"Invalid critical path value: {critical}")
    if eligible and eligible not in ALLOWED_BOOL:
        errors.append(f"Invalid jules eligible value: {eligible}")
    if status and status not in ALLOWED_STATUS:
        errors.append(f"Invalid status value: {status}")

    if not allowed_paths:
        errors.append("Allowed Paths section must contain at least one bullet")

    if lane == "JULES" and eligible != "YES":
        errors.append("JULES lane requires `Jules Eligible: YES`")

    if lane == "JULES" and critical == "YES":
        errors.append("Critical path tickets cannot run in JULES lane")

    if errors:
        print(f"Ticket validation failed for {ticket_path}:", file=sys.stderr)
        for err in errors:
            print(f"- {err}", file=sys.stderr)
        return 1

    if warnings:
        print(f"Ticket validation warnings for {ticket_path}:")
        for warning in warnings:
            print(f"- {warning}")

    print(f"Ticket validation passed: {ticket_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
