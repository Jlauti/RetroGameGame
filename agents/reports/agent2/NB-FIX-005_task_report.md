# NB-FIX-005 Task Report — Ground Read and Health Drop Hotfix

**Ticket**: NB-FIX-005
**Agent**: Pekka Kone (agent2)
**Status**: DONE
**Date**: 2026-03-08

## Summary

Implemented the ticket exactly as a Nebula-first hotfix:

1. fixed the startup procgen/runtime reset that caused overlapping and camera-relative ground/topography reads
2. preserved the approved procedural neon ground and visual-language contracts
3. made health-bearing breakable rewards visibly readable after spawn, without adding new gameplay systems

## Changes Made

### `src/eras/era_future/nebula_bouncer/systems.rs`

- Added `initialize_procgen_runtime_state(...)` and used it before chunk prefill so runtime procgen state no longer resets after pre-spawning terrain.
- Removed the duplicate startup first-chunk spawn path that caused fragmented/overlapping floor reads.
- Reworked `spawn_health_drop(...)` from a flat drop marker into a readable floating beacon built from existing meshes/materials.
- Added animated health-drop lift/pulse/rotation through `health_drop_visual_profile(...)`.
- Updated `update_health_drops(...)` to animate the spawned reward and preserve post-spawn readability.
- Added `Visibility::Visible` to the drop parent so the child meshes inherit visibility cleanly.
- Added tests:
  - `initialize_procgen_runtime_state_resets_progress_and_preserves_seed`
  - `health_drop_visual_profile_stays_lifted_and_animates`

### `src/eras/era_future/nebula_bouncer/mod.rs`

- Ordered `update_health_drops` before `handle_orb_collisions` so a newly spawned health reward is not collected in the same frame it appears.

## Acceptance Gates

All required cargo gates were rerun on the final code:

| Gate | Result |
|------|--------|
| `cargo check` | Pass |
| `cargo build --bin retro-game-game` | Pass |
| `cargo test --lib nebula_bouncer` | Pass (`42/42`) |
| `cargo fmt -- --check` | Pass |

Notes:

- cargo emitted pre-existing unrelated warnings in other eras/modules
- no gate failures remained on the final hotfix build

## BRP/MCP Validation

Validated with `RETRO_DEV_BOOT=nebula` and `BEVY_BRP_ENABLE=1`.

- Moving gameplay / world-anchored ground:
  - `agents/deliverables/agent2/NB-FIX-005_ground_start.png`
  - `agents/deliverables/agent2/NB-FIX-005_ground_move.png`
- Coherent floor/topography:
  - `agents/deliverables/agent2/NB-FIX-005_brp_validation.json`
  - startup sequence advanced cleanly from chunk centers `400.0 -> 4400.0` with no duplicate reset
- Destroyed health-bearing breakable with visible collectable:
  - `agents/deliverables/agent2/NB-FIX-005_health_drop.png`
  - runtime telemetry recorded `health_breakables_destroyed = 1`
  - live `HealthDrop` entity was present with `heal_amount = 10`

## Deliverables

- Runtime notes: `agents/deliverables/agent2/NB-FIX-005_runtime_notes.md`
- Task report: `agents/reports/agent2/NB-FIX-005_task_report.md`
- Supporting BRP evidence:
  - `agents/deliverables/agent2/NB-FIX-005_brp_validation.json`
  - `agents/deliverables/agent2/NB-FIX-005_brp_run.out.log`
  - `agents/deliverables/agent2/NB-FIX-005_brp_run.err.log`
  - `agents/deliverables/agent2/NB-FIX-005_ground_start.png`
  - `agents/deliverables/agent2/NB-FIX-005_ground_move.png`
  - `agents/deliverables/agent2/NB-FIX-005_health_drop.png`
