# NB-FIX-006 - Nebula Ground Runtime Regression Recheck

## Metadata

- Ticket ID: NB-FIX-006
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
- Routing Reason: fresh runtime pass required after HITL contradicted the prior ground/readability hotfix report
- Start Date: 2026-03-08

## Objective

Resolve the still-visible runtime regressions in Nebula's ground/boundary pass after `NB-FIX-005` failed live validation.

Required outcomes:
- ground and topography must visibly change from the broken pre-hotfix state in the shipped debug executable
- floor motifs must stop reading as overlapping, detached, or camera-relative fragments during live movement
- health-bearing breakables must produce pickups that are actually observable in ordinary live play when destroyed
- the fix must be validated against the real executable behavior the human sees, not BRP-only success signals

## Governing Inputs

- `agents/backlog/NB-FIX-005.md`
- `agents/reports/agent2/NB-FIX-005_task_report.md`
- `agents/deliverables/agent2/NB-FIX-005_runtime_notes.md`
- latest human HITL feedback from the debug executable
- `agents/backlog/NB-A1-010.md`
- `agents/deliverables/agent1/NB-A1-010_procedural_neon_ground_boundary_contract.md`
- `agents/backlog/NB-A4-013.md`
- `agents/deliverables/agent4/NB-A4-013_procedural_neon_terrain_visual_language_sheet.md`

## Allowed Paths

- `src/eras/era_future/nebula_bouncer/`
- `specs/nebula_bouncer.md`
- `docs/architecture/DESIGN.md`
- `agents/deliverables/agent2/`
- `agents/reports/agent2/`
- `agents/backlog/NB-FIX-006.md`

## Out of Scope

- No new HUD or health-bar feature work.
- No new enemy-role work.
- No new chapter/faction art direction.
- No human-authored environment model production.
- No resource economy changes.
- No non-Nebula era refactors.

## Acceptance Commands

- `cargo check`
- `cargo build --bin retro-game-game`
- `cargo test --lib nebula_bouncer`
- `cargo fmt -- --check`

## Ticket-Scoped Verification

- Boot Nebula directly with `RETRO_DEV_BOOT=nebula`.
- Enable BRP with `BEVY_BRP_ENABLE=1`.
- Validate against the same local debug executable path the human will run.
- Produce evidence for all of the following:
  - one moving gameplay case where the floor clearly reads differently from the prior broken state and remains world-anchored
  - one case showing that overlapping/disconnected floor fragments are no longer present in the active play lane
  - one live-play case where a health-bearing breakable is destroyed and its pickup is visibly present long enough to be noticed and collected
- Store a short runtime note artifact in `agents/deliverables/agent2/`.
- Include one explicit note explaining why BRP validation previously passed while HITL still failed, and what was changed to close that gap.

## Dependencies

- `NB-A1-010` completed.
- `NB-A4-013` completed.
- `NB-FIX-005` reported done but failed HITL.

## Definition of Done

- Human HITL should be able to distinguish the new floor behavior from the previously broken baseline without relying on internal telemetry.
- Ground/topography no longer appears unchanged, camera-relative, or visibly fragmented during normal play movement.
- Health-bearing breakables produce pickups that are noticeable in live play once spawned.
- No assumption is added that health bars exist; this ticket is about pickup readability and ground coherence only.
- Deliverable written to `agents/deliverables/agent2/NB-FIX-006_runtime_notes.md`.
- Task report written to `agents/reports/agent2/NB-FIX-006_task_report.md`.

## Principal Acceptance Note

- 2026-03-09 accepted for integration after lenient principal review and human HITL steering.
- The added player health HUD is acknowledged as scope drift relative to ticket wording, but it is being tolerated for this pass rather than used to block the fix.
