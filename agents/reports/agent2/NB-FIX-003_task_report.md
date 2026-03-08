# NB-FIX-003 Task Report — Enemy Approach Direction and Facing Hotfix

**Ticket**: NB-FIX-003
**Agent**: Pekka Kone (agent2)
**Status**: DONE
**Date**: 2026-03-08

## Summary

Fixed three interconnected bugs causing enemies to read as same-lane convoy traffic instead of incoming threats:

1. Enemies were tagged `ChunkMember` and scrolled by the terrain scroll system alongside floor tiles — they had no independent motion against the scroll.
2. Enemy model rotation was identical to the player ship (`Z(PI) * X(FRAC_PI_2)`), making enemies visually face in the same direction as the player rather than toward it.
3. Enemy movement speed targets were too low to overcome the 150 units/s terrain scroll, so approach velocity went negative.

## Changes Made

### `systems.rs`
- **Enemy spawn** (~line 2004): Removed `ChunkMember` from enemy entity. Added comment explaining enemies locomote independently and must not be scroll-managed.
- **Enemy model rotation** (~line 2037): Changed from `Quat::from_rotation_z(PI) * Quat::from_rotation_x(FRAC_PI_2)` to just `Quat::from_rotation_x(FRAC_PI_2)`. Enemies now face `−Y` (toward the player) vs. player facing `+Y`.
- **`enemy_movement_system`**: Completely overhauled. Now:
  - Queries `LinearVelocity` (physics-integrated) instead of mutating `Transform` directly.
  - Drives enemies to absolute world-space target Y positions: `player_pos.y + 380` (Blocker), `player_pos.y + 290` (Flanker), `player_pos.y + 620` (Sniper).
  - Uses velocity lerp-to-desired for smooth approach and braking.
  - Approach speeds: Blocker 160, Flanker 260, Sniper 90 — all sufficient to overcome/match terrain scroll (150 units/s).
  - Idle enemies brake smoothly via velocity decay.
- **`cull_behind_player_enemies`** (new): Despawns enemies >800 units behind the player Y. Releases combat tokens on cull. Replaces the ChunkMember-based scroll despawn that enemies no longer participate in.

### `mod.rs`
- Added `cull_behind_player_enemies` to the Update schedule between `enemy_movement_system` and `enemy_fire_system`.

## Acceptance Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ Pass |
| `cargo build --bin retro-game-game` | ✅ Pass |
| `cargo test --lib nebula_bouncer` | ✅ Pass (31/31) |
| `cargo fmt -- --check` | ✅ Pass |

## Deliverables

- Runtime notes: `agents/deliverables/agent2/NB-FIX-003_runtime_notes.md`
- Task report: `agents/reports/agent2/NB-FIX-003_task_report.md`
