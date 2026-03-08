# NB-FIX-004 Task Report — Enemy Presentation and World-Relative Motion Hotfix

**Ticket**: NB-FIX-004
**Agent**: Pekka Kone (agent2)
**Status**: DONE
**Date**: 2026-03-08

## Summary

Fixed three remaining enemy presentation and locomotion regressions:

1. Removed the legacy red/orange box quad artifact from enemy spawn — a leftover 2D-era aura plane that clashed with the 3D ship model.
2. Idle enemies now move with the world terrain at −150 u/s instead of braking to zero and appearing to float backward.
3. Enemy approach velocity is now grounded in world-scroll rate + bounded correction, not directly scaled by player W/S throttle.

## Changes Made

### `systems.rs`

**Enemy spawn (around line 1988):**
- Removed `glow_color`, `aura_material`, and the second `.spawn(Mesh3d(quad_mesh) ...)` child entity.
- Only the `SceneRoot` glTF model child remains.

**`ENEMY_WORLD_SCROLL_SPEED` constant (new):**
- `const ENEMY_WORLD_SCROLL_SPEED: f32 = 150.0;` added before `enemy_movement_system`, kept in sync with `update_level_scrolling`'s `SCROLL_SPEED`.

**`enemy_movement_system` (overhauled):**
- Query changed from `&mut Transform` to `&Transform` (no longer needs to write transform directly; velocity handles movement).
- **Idle**: targets `vel.y = -ENEMY_WORLD_SCROLL_SPEED`, brakes laterally. Enemies appear stationary relative to terrain.
- **Active**: sets `desired_vel.y = -ENEMY_WORLD_SCROLL_SPEED + approach_correction.y`. Approach correction is bounded (Blocker 80, Flanker 160, Sniper 40 u/s) and does not grow with player W/S input. Enemy approach reads as world-grounded motion with a mild convergence drift toward engagement depth.

## Acceptance Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ Pass (no errors) |
| `cargo build --bin retro-game-game` | ✅ Pass |
| `cargo test --lib nebula_bouncer` | ✅ Pass (31/31) |
| `cargo fmt -- --check` | ✅ Pass |

## Deliverables

- Runtime notes: `agents/deliverables/agent2/NB-FIX-004_runtime_notes.md`
- Task report: `agents/reports/agent2/NB-FIX-004_task_report.md`
