# Dispatch Now

Generated: 2026-02-19
Owner: principal_engineer

## Agent Status Snapshot

1. Aarne Tasapaino (`agent1`): idle; no active local ticket.
2. Pekka Kone (`agent2`): assigned `NB-A2-003` for runtime asset integration follow-up.
3. Ilmari Maasto (`agent3`): idle; no active local ticket.
4. Aino Kuvitus (`agent4`): assigned `NB-A4-006` art production batch 2.
5. Veikko Fiilis (`agent5`): idle; no active local ticket.
6. Codex Worker 1: `NB-CX-011` closeout/merge-prep lane.
7. Codex Worker 2: idle standby.
8. QA (`sanna_laatu`): idle; prior `NB-QA-016` complete.

## Loop Round 1 Dispatch

1. Principal Engineer
- Finalize metadata alignment for `NB-CX-009` and include in merge-ready wave.
- Keep gate queue running continuously.

2. Codex Worker 1
- Ticket: `NB-CX-011`
- Prompt: `/home/jl/git/RetroGameGame/agents/prompts/codex_worker1_kickoff_nb-cx-011.md`
- Scope: closeout evidence + gate alignment to move ticket toward merge-ready.

3. Agent 2 (Pekka Kone)
- Ticket: `NB-A2-003`
- Delegation: `/home/jl/git/RetroGameGame/agents/delegations/agent2/NB-A2-003_task.md`
- Scope: runtime asset integration hardening in `era_future`.

## Universal Gate Command

`python3 /home/jl/git/RetroGameGame/agents/scripts/gate_queue.py run-loop --wait-lock`
