#!/usr/bin/env python3
"""Validate QA signoff result for a ticket."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from agent_io import parse_metadata


def main() -> int:
    parser = argparse.ArgumentParser(description="Check QA signoff gate")
    parser.add_argument("--ticket-id", required=True, help="Ticket ID")
    parser.add_argument("--qa-file", required=True, help="Path to qa signoff markdown")
    args = parser.parse_args()

    qa_path = Path(args.qa_file)
    if not qa_path.exists():
        print(f"ERROR: QA signoff file not found: {qa_path}", file=sys.stderr)
        return 2

    meta = parse_metadata(qa_path.read_text(encoding="utf-8"))
    got_ticket = meta.get("ticket_id", "")
    gate = meta.get("gate_result", "").upper()

    if got_ticket and got_ticket != args.ticket_id:
        print(
            f"ERROR: QA ticket mismatch: expected {args.ticket_id}, found {got_ticket}",
            file=sys.stderr,
        )
        return 1

    if gate != "PASS":
        print(f"ERROR: QA gate is not PASS (found: {gate or 'missing'})", file=sys.stderr)
        return 1

    print(f"QA signoff passed for ticket {args.ticket_id}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
