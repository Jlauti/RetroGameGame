# Codex Workers - Next Dispatch

Updated: 2026-02-19
Owner: principal_engineer

## Status Check

- `codex_worker1` latest active ticket: `NB-CX-011` (READY_FOR_QA; loop closeout needed)
- `codex_worker2` status: idle
- Gate queue: no pending/running jobs after NB-CX-009 merge gate PASS

## Worker 1 Dispatch

- Ticket: `NB-CX-011`
- Prompt: `c:\Users\jlaut\git\RetroGameGame/agents/prompts/codex_worker1_kickoff_nb-cx-011.md`
- Report target: `c:\Users\jlaut\git\RetroGameGame/agents/reports/codex_worker1/NB-CX-011_task_report.md`
- Objective: confirm all evidence and gate artifacts are aligned so ticket can be promoted to merge-ready state without stale metadata.

## Worker 2 Dispatch

- Ticket: NONE (standby)
- Trigger to activate: if loop round 1 opens another parallel Nebula technical ticket requiring code implementation.

## Gate Rule

- Implementation first.
- Enqueue one `TICKET` gate job at ticket end:
  `py c:\Users\jlaut\git\RetroGameGame/agents/scripts/gate_queue.py enqueue --ticket <TICKET_ID> --mode TICKET --workdir <worktree_path> --scoped-test "<scoped command>"`
