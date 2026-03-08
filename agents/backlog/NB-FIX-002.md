# NB-FIX-002 - Nebula Enemy Fire Loop Hotfix

## Metadata

- Ticket ID: NB-FIX-002
- Owner Agent: agent2
- Recommended Specialist: Pekka Kone (Engine/Runtime)
- Preferred Model: Codex
- Complexity: Complex
- Status: DONE
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: agent2
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: HOTFIX
- Start Date: 2026-03-07
- Completed Date: 2026-03-07

## Objective

Fix the reviewed regressions in Nebula's enemy AI and hostile-fire loop so the first return-fire pass is mechanically trustworthy for HITL.

Required outcomes:
- hostile projectiles must be able to damage the player through a real, live player-damage path
- hostile damage must feed into the existing run-failure/results flow or an approved equivalent player-death path
- combat tokens must be released correctly when token-holding enemies die or otherwise leave the attack loop
- the hostile projectile cap must be enforced correctly even when multiple enemies fire in the same frame
- the `Telegraphing` state must produce a real readable telegraph that satisfies the approved enemy-combat contract

## Allowed Paths

- `src/eras/era_future/nebula_bouncer/`
- `specs/nebula_bouncer.md`
- `docs/architecture/DESIGN.md`
- `agents/deliverables/agent2/`
- `agents/reports/agent2/`
- `agents/backlog/NB-FIX-002.md`

## Out of Scope

- No chapter/faction planning changes.
- No broad enemy-behavior redesign beyond what is needed to fix the reviewed regressions.
- No non-Nebula era refactors.
- No human-authored model replacement.

## Acceptance Commands

- `cargo check`
- `cargo build --bin retro-game-game`
- `cargo test --lib nebula_bouncer`
- `cargo fmt -- --check`

## Ticket-Scoped Verification

- Boot Nebula directly with `RETRO_DEV_BOOT=nebula`.
- Enable BRP with `BEVY_BRP_ENABLE=1`.
- Use BRP/MCP-assisted runtime validation to capture:
  - one hostile projectile hit that actually affects player health/state
  - one player-death or equivalent failure-path case caused by hostile fire
  - one multi-enemy fire case confirming the simultaneous hostile-shot cap is respected
  - one telegraph case showing a visible pre-fire cue before hostile shots launch
- Store a short runtime note artifact in `agents/deliverables/agent2/`.

## Dependencies

- `NB-A1-009` completed.
- `NB-A2-013` reviewed.

## Definition of Done

- Player entities expose the runtime state needed for hostile projectiles to deal real damage.
- Hostile fire now participates in a valid player failure/results path rather than silently doing nothing.
- Combat tokens are reclaimed reliably when attackers die, despawn, or otherwise leave eligibility.
- The hostile projectile cap remains correct under same-frame multi-enemy firing.
- Enemy telegraphing is visibly readable and matches the approved contract rather than existing only as a timer.
- `NB-A2-013` is functionally superseded by this hotfix pass and can be re-marked done once the reviewed issues are resolved.
- Deliverable written to `agents/deliverables/agent2/NB-FIX-002_runtime_notes.md`.
- Task report written to `agents/reports/agent2/NB-FIX-002_task_report.md`.
