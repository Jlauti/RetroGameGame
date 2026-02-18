#!/usr/bin/env python3
"""Tests for release board construction and ordering."""

from __future__ import annotations

import json
import shutil
import tempfile
import unittest
from pathlib import Path
import sys

SCRIPTS_DIR = Path(__file__).resolve().parents[1] / "scripts"
sys.path.insert(0, str(SCRIPTS_DIR))

import build_release_board as brb  # noqa: E402


class BuildReleaseBoardTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp(prefix="build_release_board_test_"))
        self.release_dir = self.tmp / "agents/status/release"
        self.release_dir.mkdir(parents=True, exist_ok=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.tmp)

    def test_merge_order_prefers_critical_then_oldest(self) -> None:
        snapshot = {
            "generated_at": "2026-02-17T10:00:00+00:00",
            "source_root": str(self.tmp),
            "tickets": [
                {
                    "ticket_id": "NB-A",
                    "owner_agent": "agent1",
                    "critical_path": "YES",
                    "declared_status": "IN_PROGRESS",
                    "effective_status": "READY_FOR_MERGE",
                    "base_effective_status": "READY_FOR_MERGE",
                    "lane": "LOCAL",
                    "latest_report_path": None,
                    "qa_signoff_path": None,
                    "merge_manifest_path": None,
                    "latest_gate_job_id": None,
                    "evidence": ["e1"],
                    "conflicts": [],
                    "last_activity_date": "2026-02-10T00:00:00+00:00",
                },
                {
                    "ticket_id": "NB-B",
                    "owner_agent": "agent2",
                    "critical_path": "NO",
                    "declared_status": "IN_PROGRESS",
                    "effective_status": "READY_FOR_MERGE",
                    "base_effective_status": "READY_FOR_MERGE",
                    "lane": "LOCAL",
                    "latest_report_path": None,
                    "qa_signoff_path": None,
                    "merge_manifest_path": None,
                    "latest_gate_job_id": None,
                    "evidence": ["e1"],
                    "conflicts": [],
                    "last_activity_date": "2026-02-01T00:00:00+00:00",
                },
                {
                    "ticket_id": "NB-C",
                    "owner_agent": "agent3",
                    "critical_path": "YES",
                    "declared_status": "IN_PROGRESS",
                    "effective_status": "READY_FOR_MERGE",
                    "base_effective_status": "READY_FOR_MERGE",
                    "lane": "LOCAL",
                    "latest_report_path": None,
                    "qa_signoff_path": None,
                    "merge_manifest_path": None,
                    "latest_gate_job_id": None,
                    "evidence": ["e1"],
                    "conflicts": [],
                    "last_activity_date": "2026-02-12T00:00:00+00:00",
                },
            ],
        }
        snapshot_path = self.release_dir / "readiness_snapshot_2026-02-17.json"
        snapshot_path.write_text(json.dumps(snapshot), encoding="utf-8")
        out_path = self.release_dir / "release_board_2026-02-17.md"
        brb.build_board(snapshot_path, out_path)

        board = out_path.read_text(encoding="utf-8")
        pos_a = board.find("- NB-A:")
        pos_b = board.find("- NB-B:")
        pos_c = board.find("- NB-C:")
        self.assertNotEqual(pos_a, -1)
        self.assertNotEqual(pos_b, -1)
        self.assertNotEqual(pos_c, -1)

        # Critical tickets first; among them oldest first.
        self.assertLess(pos_a, pos_c)
        self.assertLess(pos_c, pos_b)


if __name__ == "__main__":
    unittest.main()
