# NB-FIX-005 - Nebula Ground Cohesion And Health-Drop Readability Hotfix

## Metadata

- Ticket ID: NB-FIX-005
- Owner Agent: agent2
- Recommended Specialist: Pekka Kone (Engine/Runtime)
- Preferred Model: Codex
- Complexity: Complex
- Status: CHANGES_REQUIRED
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: agent2
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: HOTFIX
- Routing Reason: runtime regression correction after HITL on newly integrated ground/boundary pass
- Start Date: 2026-03-08

## Objective

Fix the post-integration runtime regressions in Nebula's new ground and boundary pass so the battlefield reads as a coherent world surface instead of broken camera-relative fragments.

Required outcomes:
- ground hexes and terrain fixtures must feel anchored to the world and scroll coherently instead of appearing to overlap, separate, or move with the camera
- the new procedural-neon ground pass must preserve a stable floor read through motion rather than looking like disconnected floating pieces
- health-bearing breakable rewards must be visibly readable in real play once spawned
- the fix must stay within the approved `NB-A1-010` and `NB-A4-013` contracts without reopening the battlefield direction

## Governing Inputs

- `agents/backlog/NB-A2-015.md`
- `agents/deliverables/agent1/NB-A1-010_procedural_neon_ground_boundary_contract.md`
- `agents/deliverables/agent4/NB-A4-013_procedural_neon_terrain_visual_language_sheet.md`
- latest HITL screenshot/feedback from principal review

## Allowed Paths

- `src/eras/era_future/nebula_bouncer/`
- `specs/nebula_bouncer.md`
- `docs/architecture/DESIGN.md`
- `agents/deliverables/agent2/`
- `agents/reports/agent2/`
- `agents/backlog/NB-FIX-005.md`

## Out of Scope

- No new enemy-role work.
- No new chapter/faction art direction.
- No human-authored environment model production.
- No new resource economy or HUD/health-bar feature design.
- No non-Nebula era refactors.

## Acceptance Commands

- `cargo check`
- `cargo build --bin retro-game-game`
- `cargo test --lib nebula_bouncer`
- `cargo fmt -- --check`

## Ticket-Scoped Verification

- Boot Nebula directly with `RETRO_DEV_BOOT=nebula`.
- Enable BRP with `BEVY_BRP_ENABLE=1`.
- Validate with both BRP inspection and a real runtime capture that shows:
  - one moving gameplay case where ground surfaces remain visually world-anchored and do not appear to slide with the camera
  - one case showing the floor/topography reads as a coherent surface rather than overlapping disconnected fragments
  - one case showing a destroyed health-bearing breakable producing a visible, collectable health drop in live play
- Store a short runtime note artifact in `agents/deliverables/agent2/`.

## Dependencies

- `NB-A1-010` completed.
- `NB-A4-013` completed.
- `NB-A2-015` reviewed with HITL regressions.

## Definition of Done

- Ground/topography no longer appears camera-relative, detached, or visibly broken during normal play movement.
- The procedural-neon floor still reads as authored terrain motifs after the fix, not as random overlapping fragments.
- Health-bearing breakables produce pickups that are actually noticeable and readable in motion once destroyed.
- No assumption is added that health bars exist; this ticket only addresses reward readability and ground coherence.
- Existing motif metadata, breakable semantics, side-cage separation, and cadence logic remain intact after the hotfix.
- Deliverable written to `agents/deliverables/agent2/NB-FIX-005_runtime_notes.md`.
- Task report written to `agents/reports/agent2/NB-FIX-005_task_report.md`.

## Principal Review Follow-Up

- 2026-03-08 HITL contradicted the reported outcome from the debug executable:
  - ground still appears visually broken and unchanged from the pre-hotfix regression
  - no health pickups were observed in live play
  - no health-bar expectation is implied; missing health bars are not part of this ticket
- Further correction work has been split into `NB-FIX-006` for a fresh runtime pass.
