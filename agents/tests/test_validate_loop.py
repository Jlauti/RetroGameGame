#!/usr/bin/env python3
"""Tests for loop artifact validation."""

from __future__ import annotations

import shutil
import tempfile
import unittest
from pathlib import Path
import sys

SCRIPTS_DIR = Path(__file__).resolve().parents[1] / "scripts"
sys.path.insert(0, str(SCRIPTS_DIR))

import validate_loop as vl  # noqa: E402


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


class ValidateLoopTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp(prefix="validate_loop_test_"))

    def tearDown(self) -> None:
        shutil.rmtree(self.tmp)

    def make_loop(self, body: str) -> Path:
        path = self.tmp / "loop.md"
        write(path, body)
        return path

    def run_validate(self, path: Path) -> int:
        argv = sys.argv
        try:
            sys.argv = ["validate_loop.py", "--loop", str(path)]
            return vl.main()
        finally:
            sys.argv = argv

    def test_valid_gameplay_loop_passes(self) -> None:
        path = self.make_loop(
            """# Loop\n\n## Metadata\n\n- Loop ID: LOOP-2026-02-18-demo\n- Name: Demo\n- Owner: principal_engineer\n- Status: ACTIVE\n- Value Hypothesis: Improve gameplay quality.\n- Value Class: GAMEPLAY\n\n## Scope In\n\n- src/eras/era_future/nebula_bouncer/\n\n## Scope Out\n\n- Unrelated eras\n\n## Tickets Included\n\n- NB-CX-011\n\n## Worker Plan\n\n- principal_engineer: orchestration\n- agent2: implementation\n\n## Acceptance Commands\n\n- cargo-safe check --bin retro-game-game\n\n## Acceptance Evidence Required\n\n- qa signoff\n\n## Completion Gate\n\n- qa pass\n"""
        )
        self.assertEqual(self.run_validate(path), 0)

    def test_docs_only_non_throughput_fails(self) -> None:
        path = self.make_loop(
            """# Loop\n\n## Metadata\n\n- Loop ID: LOOP-2026-02-18-docs\n- Name: Docs Loop\n- Owner: principal_engineer\n- Status: ACTIVE\n- Value Hypothesis: Keep docs fresh.\n- Value Class: GAMEPLAY\n\n## Scope In\n\n- docs/agentic/\n- agents/status/\n\n## Scope Out\n\n- src/\n\n## Tickets Included\n\n- NB-CX-011\n\n## Worker Plan\n\n- principal_engineer: orchestration\n\n## Acceptance Commands\n\n- cargo-safe fmt -- --check\n\n## Acceptance Evidence Required\n\n- updated docs\n\n## Completion Gate\n\n- docs updated\n"""
        )
        self.assertEqual(self.run_validate(path), 1)

    def test_release_throughput_docs_loop_passes_with_queue_commands(self) -> None:
        path = self.make_loop(
            """# Loop\n\n## Metadata\n\n- Loop ID: LOOP-2026-02-18-throughput\n- Name: Throughput\n- Owner: principal_engineer\n- Status: ACTIVE\n- Value Hypothesis: Merge queue throughput improvement.\n- Value Class: RELEASE_THROUGHPUT\n\n## Scope In\n\n- docs/agentic/\n- agents/status/\n\n## Scope Out\n\n- src/\n\n## Tickets Included\n\n- NB-CX-009\n\n## Worker Plan\n\n- principal_engineer: orchestration\n\n## Acceptance Commands\n\n- python3 agents/scripts/reconcile_ticket_state.py --root /home/jl/git/RetroGameGame --date 2026-02-18\n- python3 agents/scripts/build_release_board.py --root /home/jl/git/RetroGameGame --date 2026-02-18\n- python3 agents/scripts/gate_queue.py status\n\n## Acceptance Evidence Required\n\n- readiness snapshot\n\n## Completion Gate\n\n- queue is healthy\n"""
        )
        self.assertEqual(self.run_validate(path), 0)

    def test_missing_required_metadata_fails(self) -> None:
        path = self.make_loop(
            """# Loop\n\n## Metadata\n\n- Loop ID: LOOP-2026-02-18-incomplete\n\n## Scope In\n\n- src/\n\n## Scope Out\n\n- docs/\n\n## Tickets Included\n\n- NB-CX-011\n\n## Worker Plan\n\n- principal_engineer: orchestration\n\n## Acceptance Commands\n\n- cargo-safe check\n\n## Acceptance Evidence Required\n\n- qa signoff\n\n## Completion Gate\n\n- done\n"""
        )
        self.assertEqual(self.run_validate(path), 1)


if __name__ == "__main__":
    unittest.main()
