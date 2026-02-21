# Machine Switch Protocol

Portable workflow for moving principal-engineer and agent-loop execution between PCs using Git as source-of-truth.

## Goal

A new machine should be able to:

1. `git pull`
2. Read one protocol
3. Resume the active loop without hidden local state

## Source Of Truth Contract

All execution state required for resuming must live in tracked files under `agents/` and `docs/agentic/`.

Always commit/push updates to:

- `agents/principal_engineer/current_context.md`
- `agents/principal_engineer/memory.md` (when decisions change)
- `agents/status/next_assignments.md`
- `agents/status/dispatch_now.md`
- `agents/status/current_milestone.md` (if milestone state changed)
- `agents/loops/<LOOP_ID>.md`
- ticket artifacts (`agents/backlog/`, `agents/delegations/`, `agents/reports/`, `agents/qa/`, `agents/merge/`)
- readiness artifacts (`agents/status/release/readiness_snapshot_<DATE>.json`, `agents/status/release/release_board_<DATE>.md`)
- queue artifacts (`agents/status/gates/queue/pending.jsonl`, `agents/status/gates/queue/running.json`, `agents/status/gates/queue/history/*.json`)

Do not rely on untracked local notes for operational decisions.

## Leave-Machine Checklist

1. Ensure queue state is explicit:
   - If work should continue elsewhere, keep jobs in `pending.jsonl` and commit.
   - If no active gate run, `running.json` must be `{}`.
2. Refresh reconciliation artifacts:
   - `python3 agents/scripts/reconcile_ticket_state.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
   - `python3 agents/scripts/build_release_board.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
3. Refresh principal context:
   - `python3 agents/scripts/update_principal_context.py`
4. Update active loop next actions in `agents/loops/<LOOP_ID>.md`.
5. Commit and push all operational-state changes to `develop`.

## Asset Sync Gate (Required)

Before switching machines, confirm there are no forgotten local-only assets.

Audit commands:

```bash
git status --short -- assets agents/art specs/future/nebula_bouncer
git ls-files --others --exclude-standard assets agents/art specs/future/nebula_bouncer
```

If output is non-empty, either:

1. commit required files, or
2. explicitly discard files that are confirmed out-of-scope for the active loop.

Keep canonical runtime assets and art-review evidence in Git so asset-driven tickets can resume on any machine.

## Arrive-On-New-Machine Checklist

1. Pull latest:
   - `git checkout develop`
   - `git pull --ff-only`
2. Read in order:
   - `agents/principal_engineer/launch_prompt.md`
   - `agents/principal_engineer/OPERATING_PROTOCOL.md`
   - `agents/principal_engineer/current_context.md`
   - active `agents/loops/<LOOP_ID>.md`
   - latest `agents/status/release/release_board_<DATE>.md`
3. Validate loop:
   - `python3 agents/scripts/validate_loop.py --loop agents/loops/<LOOP_ID>.md`
4. Resume gates if needed:
   - `python3 agents/scripts/gate_queue.py run-loop --wait-lock`
5. Continue dispatch from:
   - `agents/status/next_assignments.md`
   - `agents/status/dispatch_now.md`

## Queue Ownership Rules

- Exactly one active gate runner across all machines.
- Before starting `run-loop`, check:
  - `python3 agents/scripts/gate_queue.py status`
  - `agents/status/gates/queue/running.json`
- If `running.json` is non-empty but no runner is actually alive, reconcile explicitly and commit the corrected state.

## Recovery If Machines Diverge

1. Pause new dispatch.
2. Keep `develop` authoritative.
3. Reconcile ticket state and release board from current repo artifacts.
4. Update `agents/principal_engineer/current_context.md` with explicit decisions and ownership.
5. Resume loop only after queue, assignments, and loop artifact agree.
