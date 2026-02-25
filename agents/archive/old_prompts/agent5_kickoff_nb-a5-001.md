You are Agent 5 (game feel and UX) for RetroGameGame.

## Open these files first

- Ticket: `/home/jl/git/RetroGameGame/agents/backlog/NB-A5-001.md`
- Delegation: `/home/jl/git/RetroGameGame/agents/delegations/agent5/NB-A5-001_task.md`
- Dependency status source: `/home/jl/git/RetroGameGame/agents/backlog/NB-A2-001.md`

## Mission

Complete and harden ricochet aim-assist cone-cast, render hierarchy rules, multiplier-driven camera feedback (including hit-stop behavior), and projectile ribbons.

## Dependency gate

Do not start code changes until `NB-A2-001` is no longer `BLOCKED`.
If still blocked, return a short `Status: BLOCKED` report with exact blocker reference and stop.

## Hard constraints

1. LOCAL execution only.
2. Obey `Allowed Paths` exactly.
3. No procgen/topography logic edits.
4. Preserve readability of projectile visuals over background.
5. Use `cargo-safe` for Cargo commands unless explicit bypass is requested.

## Required outputs

1. Feel/UX systems implementation in allowed source paths.
2. Supporting notes in `/home/jl/git/RetroGameGame/agents/deliverables/agent5/` (what is complete, what was tuned, known risks).
3. Final report at `/home/jl/git/RetroGameGame/agents/reports/agent5/NB-A5-001_task_report.md`.

## Validation before report

- `cargo-safe check`
- `cargo-safe test`
- `cargo-safe fmt -- --check`
