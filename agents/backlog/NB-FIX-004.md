# NB-FIX-004 - Nebula Enemy Presentation and World-Relative Motion Hotfix

## Metadata

- Ticket ID: NB-FIX-004
- Owner Agent: agent2
- Recommended Specialist: Pekka Kone (Engine/Runtime)
- Preferred Model: Codex
- Complexity: Complex
- Status: TODO
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: agent2
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: HOTFIX
- Start Date: 2026-03-08

## Objective

Fix the remaining enemy presentation and locomotion regressions in Nebula so enemies read cleanly as incoming world-space threats.

Required outcomes:
- remove the legacy red box visual still attached to enemy presentation
- enemies must not visually drift backward by default before they fully engage
- enemy motion must be grounded in world/terrain flow rather than directly keying off player `W`/`S` movement
- the fix must preserve the improved incoming-threat read from `NB-FIX-003`

## Allowed Paths

- `src/eras/era_future/nebula_bouncer/`
- `specs/nebula_bouncer.md`
- `docs/architecture/DESIGN.md`
- `agents/deliverables/agent2/`
- `agents/reports/agent2/`
- `agents/backlog/NB-FIX-004.md`

## Out of Scope

- No ground/topography redesign in this ticket.
- No new enemy role design.
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
  - one case confirming the legacy red enemy box is gone in gameplay
  - one case showing idle/pre-engagement enemies no longer reading as backward-drifting traffic
  - one case showing enemy approach motion remains stable when the player changes speed with `W`/`S`
- Store a short runtime note artifact in `agents/deliverables/agent2/`.

## Dependencies

- `NB-FIX-002` completed.
- `NB-FIX-003` completed.

## Definition of Done

- Enemy visuals no longer include the leftover red 2D-era box/panel artifact.
- Enemies no longer drift backward by default before active engagement in a way that breaks the incoming-threat read.
- Enemy world motion is no longer directly modulated by player throttle input; `W`/`S` changes should not make enemies appear to speed up or slow down relative to the ground in a broken way.
- Existing hostile-fire, telegraph, and damage behavior remain functional after the movement/presentation fix.
- Deliverable written to `agents/deliverables/agent2/NB-FIX-004_runtime_notes.md`.
- Task report written to `agents/reports/agent2/NB-FIX-004_task_report.md`.
