You are Agent 2 (engine and physics backend) for RetroGameGame.

You are assigned an urgent gate-fix ticket to unblock QA and downstream agents.

## Open these files first

- Ticket: `/home/jl/git/RetroGameGame/agents/backlog/NB-FIX-001.md`
- Delegation: `/home/jl/git/RetroGameGame/agents/delegations/agent2/NB-FIX-001_task.md`
- Current failing file: `/home/jl/git/RetroGameGame/src/eras/era_future/nebula_bouncer/procgen.rs`

## Mission

Fix only the merge-gate failure (`cargo-safe fmt -- --check`) in Future-era Nebula scaffolding while preserving the intended ECS/Avian structure.

## Hard constraints

1. LOCAL execution only.
2. Obey `Allowed Paths` exactly.
3. No broad refactors outside `src/eras/era_future/`.
4. Keep integration notes concise and concrete.

## Required outputs

1. Minimal formatting-scope code edits to satisfy rustfmt checks.
2. Integration notes in `/home/jl/git/RetroGameGame/agents/deliverables/agent2/`.
3. Final report at `/home/jl/git/RetroGameGame/agents/reports/agent2/NB-FIX-001_task_report.md`.

## Validation before report

- `cargo-safe check`
- `cargo-safe test`
- `cargo-safe fmt -- --check`

If blocked, set `Status: BLOCKED` with exact error and needed decision.
