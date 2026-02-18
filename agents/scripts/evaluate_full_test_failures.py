#!/usr/bin/env python3
"""Evaluate cargo test failures against a known baseline allowlist."""

from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


FAILURE_BLOCK_RE = re.compile(r"^----\s+(.+?)\s+stdout\s+----\s*$")
FAILURE_LIST_RE = re.compile(r"^\s{4}([A-Za-z0-9_:]+)\s*$")


def now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def parse_failures_from_log(text: str) -> list[str]:
    failures: list[str] = []
    in_failures_section = False

    for line in text.splitlines():
        match = FAILURE_BLOCK_RE.match(line)
        if match:
            failures.append(match.group(1).strip())
            continue

        if line.strip() == "failures:":
            in_failures_section = True
            continue

        if in_failures_section:
            if not line.strip():
                continue
            list_match = FAILURE_LIST_RE.match(line)
            if list_match:
                failures.append(list_match.group(1).strip())
                continue
            # End failures section when hitting a non-indented line.
            if not line.startswith("    "):
                in_failures_section = False

    # Keep deterministic order while removing duplicates.
    seen = set()
    ordered: list[str] = []
    for item in failures:
        if item and item not in seen:
            seen.add(item)
            ordered.append(item)
    return ordered


def load_baseline(path: Path) -> set[str]:
    if not path.exists():
        return set()
    data = json.loads(path.read_text(encoding="utf-8"))
    entries = data.get("known_failures", [])
    ids: set[str] = set()
    for entry in entries:
        if isinstance(entry, str):
            ids.add(entry.strip())
        elif isinstance(entry, dict):
            test_id = str(entry.get("test_id", "")).strip()
            if test_id:
                ids.add(test_id)
    return ids


def main() -> int:
    parser = argparse.ArgumentParser(description="Compare full-test failures against baseline list.")
    parser.add_argument("--log", required=True, help="Path to cargo test log file")
    parser.add_argument("--baseline", required=True, help="Path to baseline_failures.json")
    parser.add_argument("--out", help="Optional output JSON artifact path")
    args = parser.parse_args()

    log_path = Path(args.log)
    baseline_path = Path(args.baseline)
    if not log_path.exists():
        print(f"ERROR: log file not found: {log_path}", file=sys.stderr)
        return 2

    log_text = log_path.read_text(encoding="utf-8", errors="replace")
    failures = parse_failures_from_log(log_text)
    baseline_ids = load_baseline(baseline_path)

    known = sorted([f for f in failures if f in baseline_ids])
    new = sorted([f for f in failures if f not in baseline_ids])

    result: dict[str, Any] = {
        "evaluated_at": now_iso(),
        "log_file": str(log_path),
        "baseline_file": str(baseline_path),
        "failure_count": len(failures),
        "failures": failures,
        "known_failures": known,
        "new_failures": new,
        "only_known_failures": len(failures) > 0 and len(new) == 0,
    }

    output = json.dumps(result, indent=2, ensure_ascii=True)
    print(output)

    if args.out:
        out_path = Path(args.out)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(output + "\n", encoding="utf-8")

    if len(new) > 0:
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
