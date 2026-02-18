# Agent Control Plane

Operational source-of-truth for the project agent loop.

## Scope

`agents/` stores live execution artifacts (tickets, delegations, reports, QA, merge, status) and automation scripts.

Stable explanatory process docs are in `docs/agentic/`.

## Core Operational Paths

- `agents/backlog/`: ticket definitions
- `agents/delegations/`: task briefs by agent
- `agents/reports/`: agent completion reports
- `agents/qa/`: QA signoff artifacts
- `agents/merge/`: merge manifests
- `agents/status/`: gates, readiness snapshots, release boards, assignment state
- `agents/loops/`: milestone loop contracts and completion state
- `agents/templates/`: canonical templates
- `agents/scripts/`: orchestration + validation scripts

## Primary Process Documents

- Loop contract: `docs/agentic/AGENTIC_LOOP.md`
- Activation matrix: `docs/agentic/WORKER_ACTIVATION_MATRIX.md`
- Control-plane map: `docs/agentic/CONTROL_PLANE_MAP.md`
- Principal runbook: `docs/agentic/RUNBOOK.md`

## Principal Engineer Files

- `agents/principal_engineer/launch_prompt.md`
- `agents/principal_engineer/OPERATING_PROTOCOL.md`
- `agents/principal_engineer/memory.md`
- `agents/principal_engineer/current_context.md`

## Loop Workflow

1. Create/update loop artifact in `agents/loops/<LOOP_ID>.md`.
2. Validate loop artifact:
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/validate_loop.py --loop /home/jl/git/RetroGameGame/agents/loops/<LOOP_ID>.md`
3. Dispatch/execute tickets in scope.
4. Run queue-driven gates and QA.
5. Mark loop `COMPLETE` only after completion gate evidence is satisfied.

## Key Automation

- Queue merge-ready tickets:
  - `python3 /home/jl/git/RetroGameGame/agents/scripts/queue_merge_ready.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
- Run gate queue loop:
  - `python3 /home/jl/git/RetroGameGame/agents/scripts/gate_queue.py run-loop --wait-lock`
- Reconcile readiness:
  - `python3 /home/jl/git/RetroGameGame/agents/scripts/reconcile_ticket_state.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
- Build release board:
  - `python3 /home/jl/git/RetroGameGame/agents/scripts/build_release_board.py --root /home/jl/git/RetroGameGame --date $(date +%F)`

## Test Commands (Control Plane)

```bash
bash /home/jl/git/RetroGameGame/agents/scripts/policy_smoke_test.sh
python3 -m unittest discover -s /home/jl/git/RetroGameGame/agents/tests -p 'test_*.py' -v
```
