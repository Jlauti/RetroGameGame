# Principal Engineer Operating Protocol

Durable orchestration protocol for the principal engineer role.

## Mission

- Convert CTO intent into executable loops and tickets.
- Keep delivery moving with local-first execution.
- Preserve quality via strict QA and merge gates.
- Close loops only on testable, non-trivial value outcomes.

## Branching Discipline

- `develop` is the default integration branch.
- Ticket branches are `codex/<ticket-or-scope>` cut from `develop`.
- Ticket merges target `develop`.
- `main` is promotion-only for release-ready snapshots.
- Principal engineer is sole merger to `main`.

## Session Bootstrap

Read in order:

1. `agents/principal_engineer/memory.md`
2. `agents/principal_engineer/current_context.md`
3. Active loop artifact in `agents/loops/`
4. `agents/status/current_milestone.md`
5. Latest release board in `agents/status/release/`

Refresh context:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/update_principal_context.py
```

Validate loop artifact:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/validate_loop.py --loop /home/jl/git/RetroGameGame/agents/loops/<LOOP_ID>.md
```

## Loop Contract

Canonical loop rules:

- `docs/agentic/AGENTIC_LOOP.md`
- `docs/agentic/WORKER_ACTIVATION_MATRIX.md`

Operational loop artifact path:

- `agents/loops/<LOOP_ID>.md`

## Delegation Model

- Primary lane is `LOCAL`.
- One-ticket WIP per agent.
- Operational source-of-truth is under `agents/`.
- Worker activation follows explicit matrix (not ad-hoc staffing).

## Review And Gates

Before ticket merge to `develop`:

1. Scope boundary check
2. `cargo-safe check`
3. Ticket scoped test command
4. `cargo-safe fmt -- --check`
5. QA signoff `PASS`

Before promotion merge to `main`:

1. Scope boundary check
2. `cargo-safe check`
3. `cargo-safe test`
4. `cargo-safe fmt -- --check`
5. QA signoff `PASS`

Gate helper:

```bash
bash /home/jl/git/RetroGameGame/agents/scripts/verify_merge_gate.sh <TICKET_ID>
```

## Aggressive Merge + Cleanup Routine

Run continuously during integration windows:

1. Refresh readiness and queue merge-ready non-conflicting work:
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/queue_merge_ready.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
2. Keep one gate runner alive:
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/gate_queue.py run-loop --wait-lock`
3. Reconcile and rebuild release board after merge waves:
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/reconcile_ticket_state.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/build_release_board.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
4. Run workspace hygiene audit:
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/cleanup_workspace_dirs.py --root /home/jl/git/RetroGameGame`
5. Prune only with explicit approval:
   - `python3 /home/jl/git/RetroGameGame/agents/scripts/cleanup_workspace_dirs.py --root /home/jl/git/RetroGameGame --prune-safe`

## Loop Completion Rule

A loop is marked `COMPLETE` only when:

- loop completion gate in `agents/loops/<LOOP_ID>.md` is satisfied
- required commands pass
- required QA decisions are `PASS`
- required evidence artifacts are present

## Cargo Policy

Use `cargo-safe` by default for all Cargo subcommands.
Use plain `cargo` only with explicit bypass intent.

## Handoff Protocol

At session end, update:

1. `agents/principal_engineer/current_context.md`
2. active loop artifact status and next actions
3. `agents/status/current_milestone.md` (if changed)
4. daily status artifact if part of current workflow
5. asset sync audit before handoff:
   - `git status --short -- assets agents/art specs/future/nebula_bouncer`
   - `git ls-files --others --exclude-standard assets agents/art specs/future/nebula_bouncer`
6. follow `docs/agentic/MACHINE_SWITCH.md` before switching PCs
