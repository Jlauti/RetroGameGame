# Codex Workers - Next Dispatch

Updated: 2026-02-16
Owner: principal_engineer

## Status Check

- `codex_worker1` last report: `NB-CX-005` -> `PASS`
- `codex_worker2` last report: `NB-CX-006` -> `PASS`
- Gate queue currently has no running/pending jobs.

## Worker 1

- Ticket: `NB-CX-007`
- Prompt: `/home/jl/git/RetroGameGame/agents/prompts/codex_worker_session_1_nb-cx-007.md`
- Report target: `/home/jl/git/RetroGameGame/agents/reports/codex_worker1/NB-CX-007_task_report.md`

## Worker 2

- Ticket: `NB-CX-008`
- Prompt: `/home/jl/git/RetroGameGame/agents/prompts/codex_worker_session_2_nb-cx-008.md`
- Report target: `/home/jl/git/RetroGameGame/agents/reports/codex_worker2/NB-CX-008_task_report.md`

## Gate Rule

- Implementation first.
- Enqueue one `TICKET` gate job at ticket end:
  `python3 /home/jl/git/RetroGameGame/agents/scripts/gate_queue.py enqueue --ticket <TICKET_ID> --mode TICKET --workdir <worktree_path> --scoped-test "<scoped command>"`
