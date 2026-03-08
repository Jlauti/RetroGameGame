# NB-FIX-003 - Nebula Enemy Approach Direction and Facing Hotfix

## Metadata

- Ticket ID: NB-FIX-003
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
- Completed Date: 2026-03-08

## Objective

Fix the enemy approach-direction regression in Nebula so enemies read as incoming threats rather than player-aligned traffic.

Required outcomes:
- enemies must face and visually read as attacking from ahead of the player rather than backwards or side-slid
- enemy movement must produce opposing approach pressure inside the forward play space instead of drifting with the player like same-lane cars
- enemy spawning, locomotion, and orientation must stay consistent with the approved forward-arc combat contract
- the fix must preserve the existing hostile-fire loop, telegraph readability, and terrain/boundary integration

## Allowed Paths

- `src/eras/era_future/nebula_bouncer/`
- `specs/nebula_bouncer.md`
- `docs/architecture/DESIGN.md`
- `agents/deliverables/agent2/`
- `agents/reports/agent2/`
- `agents/backlog/NB-FIX-003.md`

## Out of Scope

- No new enemy role design.
- No broad balance pass beyond what is needed to restore correct approach direction and facing.
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
  - one case showing enemies spawning and remaining in the forward combat arc instead of trailing the player
  - one case showing enemy orientation/facing reads correctly during approach and attack setup
  - one case showing incoming relative motion against the player/scroll direction rather than same-lane convoy motion
- Store a short runtime note artifact in `agents/deliverables/agent2/`.

## Dependencies

- `NB-A1-009` completed.
- `NB-FIX-002` completed.

## Definition of Done

- Enemies no longer read as backwards-facing during approach or engagement.
- Enemies no longer move with the player as if sharing the same lane flow; they present readable incoming pressure from ahead.
- Spawning and movement stay inside the intended forward combat space and do not create behind-the-player attacks.
- Existing hostile-fire, telegraph, and damage behavior remain functional after the movement/facing correction.
- Deliverable written to `agents/deliverables/agent2/NB-FIX-003_runtime_notes.md`.
- Task report written to `agents/reports/agent2/NB-FIX-003_task_report.md`.
