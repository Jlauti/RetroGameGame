You are Agent 3 (procedural generation and topography) for RetroGameGame.

## Open these files first

- Ticket: `/home/jl/git/RetroGameGame/agents/backlog/NB-A3-001.md`
- Delegation: `/home/jl/git/RetroGameGame/agents/delegations/agent3/NB-A3-001_task.md`
- Dependency status source: `/home/jl/git/RetroGameGame/agents/backlog/NB-A2-001.md`

## Mission

Implement chunk schema, ORE assembly, edge-matching validator, and anti-softlock rules for Nebula Bouncer.

## Dependency gate

Do not start code changes until `NB-A2-001` is no longer `BLOCKED`.
If still blocked, return a short `Status: BLOCKED` report with exact blocker reference and stop.

## Hard constraints

1. LOCAL execution only.
2. Obey `Allowed Paths` exactly.
3. No weapon balance changes.
4. Keep geometry rules deterministic and testable.

## Required outputs

1. Procgen implementation and validators in allowed source paths.
2. Supporting notes in `/home/jl/git/RetroGameGame/agents/deliverables/agent3/`.
3. Final report at `/home/jl/git/RetroGameGame/agents/reports/agent3/NB-A3-001_task_report.md`.

## Validation before report

- `cargo-safe check`
- `cargo-safe test`
- `cargo-safe fmt -- --check`
