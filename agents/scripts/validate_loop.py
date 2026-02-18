#!/usr/bin/env python3
"""Validate loop artifacts for completeness and non-trivial value policy."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

from agent_io import parse_metadata, parse_section_bullets

REQUIRED = ["loop_id", "name", "owner", "status", "value_hypothesis", "value_class"]
ALLOWED_STATUS = {"PLANNED", "ACTIVE", "READY_FOR_VALIDATION", "COMPLETE", "BLOCKED"}
ALLOWED_VALUE_CLASS = {
    "GAMEPLAY",
    "ASSETIZATION",
    "STABILITY",
    "PERFORMANCE",
    "RELEASE_THROUGHPUT",
    "MIXED",
}
TICKET_RE = re.compile(r"^[A-Z]+-[A-Z0-9]+-\d+$")


def is_docs_or_control_plane_only(pathish: str) -> bool:
    value = pathish.strip().strip("`").lower()
    return value.startswith("docs/") or value.startswith("agents/")


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate a loop artifact")
    parser.add_argument("--loop", required=True, help="Path to loop markdown file")
    args = parser.parse_args()

    loop_path = Path(args.loop)
    if not loop_path.exists():
        print(f"ERROR: loop file not found: {loop_path}", file=sys.stderr)
        return 2

    text = loop_path.read_text(encoding="utf-8")
    meta = parse_metadata(text)

    scope_in = parse_section_bullets(text, "Scope In")
    scope_out = parse_section_bullets(text, "Scope Out")
    tickets = parse_section_bullets(text, "Tickets Included")
    worker_plan = parse_section_bullets(text, "Worker Plan")
    acceptance_commands = parse_section_bullets(text, "Acceptance Commands")
    acceptance_evidence = parse_section_bullets(text, "Acceptance Evidence Required")
    completion_gate = parse_section_bullets(text, "Completion Gate")

    errors: list[str] = []

    for key in REQUIRED:
        if not meta.get(key):
            errors.append(f"Missing metadata key: {key}")

    status = meta.get("status", "").strip().upper()
    value_class = meta.get("value_class", "").strip().upper()

    if status and status not in ALLOWED_STATUS:
        errors.append(f"Invalid status value: {status}")
    if value_class and value_class not in ALLOWED_VALUE_CLASS:
        errors.append(f"Invalid value class: {value_class}")

    if not scope_in:
        errors.append("Scope In section must contain at least one bullet")
    if not scope_out:
        errors.append("Scope Out section must contain at least one bullet")
    if not tickets:
        errors.append("Tickets Included section must contain at least one bullet")
    if not worker_plan:
        errors.append("Worker Plan section must contain at least one bullet")
    if not acceptance_commands:
        errors.append("Acceptance Commands section must contain at least one bullet")
    if not acceptance_evidence:
        errors.append("Acceptance Evidence Required section must contain at least one bullet")
    if not completion_gate:
        errors.append("Completion Gate section must contain at least one bullet")

    if tickets:
        invalid = [t for t in tickets if not TICKET_RE.match(t.strip())]
        if invalid:
            errors.append(f"Invalid ticket id(s): {', '.join(invalid)}")

    if worker_plan:
        has_principal = any("principal_engineer" in row.lower() for row in worker_plan)
        if not has_principal:
            errors.append("Worker Plan must include principal_engineer")

    # Non-trivial loop policy.
    if scope_in:
        only_docs_control = all(is_docs_or_control_plane_only(item) for item in scope_in)
        if only_docs_control and value_class != "RELEASE_THROUGHPUT":
            errors.append(
                "Non-trivial rule failed: docs/control-plane-only scope requires Value Class RELEASE_THROUGHPUT"
            )

    if value_class == "RELEASE_THROUGHPUT":
        command_blob = "\n".join(acceptance_commands).lower()
        if not any(
            token in command_blob
            for token in ("gate_queue", "reconcile_ticket_state", "build_release_board")
        ):
            errors.append(
                "RELEASE_THROUGHPUT loops must include queue/reconcile/release-board acceptance commands"
            )

    if errors:
        print(f"Loop validation failed for {loop_path}:", file=sys.stderr)
        for err in errors:
            print(f"- {err}", file=sys.stderr)
        return 1

    print(f"Loop validation passed: {loop_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
