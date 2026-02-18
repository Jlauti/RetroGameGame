#!/usr/bin/env python3
"""Tests for release-readiness reconciliation."""

from __future__ import annotations

import argparse
import json
import shutil
import tempfile
import unittest
from pathlib import Path
import sys

SCRIPTS_DIR = Path(__file__).resolve().parents[1] / "scripts"
sys.path.insert(0, str(SCRIPTS_DIR))

import reconcile_ticket_state as rts  # noqa: E402


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


class ReconcileTicketStateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = Path(tempfile.mkdtemp(prefix="reconcile_ticket_state_test_"))
        self.root = self.tmp
        for rel in (
            "agents/backlog",
            "agents/reports/agent1",
            "agents/qa",
            "agents/merge",
            "agents/status/gates/queue/history",
        ):
            (self.root / rel).mkdir(parents=True, exist_ok=True)

    def tearDown(self) -> None:
        shutil.rmtree(self.tmp)

    def add_ticket(
        self,
        ticket_id: str,
        status: str | None,
        owner: str = "agent1",
        lane: str = "LOCAL",
        critical: str = "YES",
    ) -> None:
        status_line = f"- Status: {status}\n" if status is not None else ""
        write(
            self.root / f"agents/backlog/{ticket_id}.md",
            (
                "# Ticket\n\n## Metadata\n\n"
                f"- Ticket ID: {ticket_id}\n"
                f"- Owner Agent: {owner}\n"
                f"{status_line}"
                f"- Execution Lane: {lane}\n"
                f"- Critical Path: {critical}\n"
            ),
        )

    def add_report(self, ticket_id: str, status: str) -> None:
        write(
            self.root / f"agents/reports/agent1/{ticket_id}_task_report.md",
            (
                "# Task Report\n\n## Metadata\n\n"
                f"- Ticket ID: {ticket_id}\n"
                "- Agent: agent1\n"
                f"- Status: {status}\n"
                "- Report Date: 2026-02-17\n"
            ),
        )

    def add_qa(self, ticket_id: str, gate_result: str) -> None:
        write(
            self.root / f"agents/qa/{ticket_id}_qa_signoff.md",
            (
                "# QA Signoff\n\n## Metadata\n\n"
                f"- Ticket ID: {ticket_id}\n"
                "- Agent: qa\n"
                "- Date: 2026-02-17\n"
                f"- Gate Result: {gate_result}\n"
            ),
        )

    def add_merge(
        self,
        ticket_id: str,
        gate_status: str = "PASS",
        ready: str = "YES",
        merge_decision: str = "UNSET",
    ) -> None:
        write(
            self.root / f"agents/merge/{ticket_id}_merge_manifest.md",
            (
                f"# Merge Manifest - {ticket_id}\n\n## Metadata\n\n"
                f"- Ticket ID: {ticket_id}\n"
                f"- Gate Status: {gate_status}\n"
                f"- Ready For Merge To Main Now: {ready}\n"
                f"- Merge Decision: {merge_decision}\n"
            ),
        )

    def add_gate(self, ticket_id: str, status: str, mode: str = "TICKET") -> None:
        payload = {
            "job_id": f"JOB-{ticket_id}-{status}",
            "ticket_id": ticket_id,
            "mode": mode,
            "status": status,
            "finished_at": "2026-02-17T12:00:00+00:00",
        }
        write(
            self.root / f"agents/status/gates/queue/history/{ticket_id}_{status}.json",
            json.dumps(payload),
        )

    def test_reconcile_scenarios(self) -> None:
        # 1) TODO + report PASS + no QA => READY_FOR_QA
        self.add_ticket("NB-T1", "TODO")
        self.add_report("NB-T1", "PASS")

        # 2) IN_PROGRESS + QA PASS + merge ready + gate PASS => READY_FOR_MERGE
        self.add_ticket("NB-T2", "IN_PROGRESS")
        self.add_report("NB-T2", "PASS")
        self.add_qa("NB-T2", "PASS")
        self.add_merge("NB-T2", gate_status="PASS", ready="YES", merge_decision="UNSET")
        self.add_gate("NB-T2", "PASS")

        # 3) merge decision MERGED => MERGED
        self.add_ticket("NB-T3", "TODO")
        self.add_merge("NB-T3", gate_status="PASS", ready="YES", merge_decision="MERGED")

        # 4) missing report + missing QA + TODO => IN_PROGRESS
        self.add_ticket("NB-T4", "TODO")

        # 5) conflicting artifacts => STALE_METADATA
        self.add_ticket("NB-T5", "READY_FOR_MERGE")
        self.add_report("NB-T5", "PASS")
        self.add_qa("NB-T5", "PASS")
        self.add_merge("NB-T5", gate_status="PASS", ready="YES", merge_decision="UNSET")
        self.add_gate("NB-T5", "FAIL")

        # 6) malformed metadata => BLOCKED
        self.add_ticket("NB-T6", None)

        out_path, rows = rts.reconcile(
            argparse.Namespace(root=str(self.root), date="2026-02-17")
        )
        self.assertTrue(out_path.exists(), "snapshot should be generated")
        by_id = {row["ticket_id"]: row for row in rows}

        self.assertEqual(by_id["NB-T1"]["effective_status"], "READY_FOR_QA")
        self.assertEqual(by_id["NB-T2"]["effective_status"], "READY_FOR_MERGE")
        self.assertEqual(by_id["NB-T3"]["effective_status"], "MERGED")
        self.assertEqual(by_id["NB-T4"]["effective_status"], "IN_PROGRESS")
        self.assertEqual(by_id["NB-T5"]["effective_status"], "STALE_METADATA")
        self.assertGreater(len(by_id["NB-T5"]["conflicts"]), 0)
        self.assertEqual(by_id["NB-T6"]["effective_status"], "BLOCKED")
        self.assertTrue(
            any(str(item).startswith("metadata_missing=") for item in by_id["NB-T6"]["evidence"])
        )


if __name__ == "__main__":
    unittest.main()
