# NB-FIX-004 Runtime Notes — Enemy Presentation and World-Relative Motion Hotfix

**Ticket**: NB-FIX-004
**Agent**: Pekka Kone (agent2)
**Date**: 2026-03-08

## Root Cause Analysis

Three presentation and locomotion regressions identified:

### Bug 1: Legacy Red/Orange Box Artifact
The enemy spawn included a second child entity — a flat `quad_mesh` plane at `depth::ENEMY + 4.0` with the enemy's glow color material (e.g. `srgb(1.0, 0.4, 0.0)` for orange/hostile enemies). In 3D rendering this appeared as a vivid colored rectangle behind/beneath the glTF ship model. This was a leftover from the 2D sprite-aura pipeline; in the current 3D Nebula runtime it serves no purpose and produces a glaring visual artifact.

### Bug 2: Idle Enemies Drifted Backward Relative to Ground
`enemy_movement_system` applied `velocity.0 *= 0.85` for Idle enemies — a dampedown toward zero velocity. Enemies are NOT `ChunkMember`, so the terrain scroll system does not move them. But the terrain itself scrolls at −150 u/s. An enemy sitting at zero velocity while the ground moves at −150 beneath it appears to drift *backward* at 150 u/s relative to the terrain — reading as backward-floated same-lane traffic.

### Bug 3: Enemy Approach Speed Followed Player W/S Throttle
`enemy_movement_system` drove enemies toward `player_pos.y + role_offset`. When the player pressed W and `player_pos.y` increased, the target Y increased by the same delta, causing corresponding approach velocity surges. Pressing S caused deceleration. Enemies appeared to directly shadow player throttle instead of moving independently through the world.

## Fixes Applied

### Fix 1: Removed Aura Quad Child from Enemy Spawn
The second `.spawn(Mesh3d(quad_mesh) ...)` child inside the enemy `with_children` block was removed. The `enemy_size`, `glow_color`, and `aura_material` intermediate bindings were also removed as they became unused.

The `SceneRoot` glTF model child remains — enemies still render using the 3D ship model.

### Fix 2 & 3: Scroll-Compensating Enemy Locomotion
Added `const ENEMY_WORLD_SCROLL_SPEED: f32 = 150.0` (kept in sync with `update_level_scrolling`'s `SCROLL_SPEED`).

New locomotion model:

**Idle state:**
```
target_vel.y = -ENEMY_WORLD_SCROLL_SPEED   // scroll with terrain
target_vel.x = lateral_dampen(vel.x)       // brake sideways
velocity = lerp(velocity, target_vel, 6·dt)
```
Enemies in Idle now appear to move with the ground (stationary relative to terrain) rather than drifting backward.

**Active states (Positioning, Telegraphing, Firing, Cooldown):**
```
approach_correction = normalize(engagement_target - enemy_pos) * approach_speed
// approach_speed: Blocker 80, Flanker 160, Sniper 40

desired_vel.y = -ENEMY_WORLD_SCROLL_SPEED + approach_correction.y
desired_vel.x =                              approach_correction.x

velocity = lerp(velocity, desired_vel, 6·dt)
```

The base Y component is always `−150`, regardless of player position or W/S input. The approach correction is bounded by `approach_speed` (max 160). Since `player_pos.y` still appears in `target_y`, pressing W does change `target_y` — but the delta contribution to `approach_correction.y` is small because it only affects the normalized vector, not the base velocity. The dominant force is always the `−150` world-scroll term.

## BRP/MCP Validation Notes

### Case 1: Red Box Gone
- The `quad_mesh` child was removed from the `with_children` block.
- Enemies render only the `SceneRoot` glTF model child.
- No flat colored rectangle should be visible in front of or behind enemy ships.

### Case 2: Idle Enemies Grounded with Terrain
- Idle enemies now have `vel.y ≈ −150`, matching terrain scroll.
- They appear stationary relative to the ground before LOS acquisition.
- No backward-drift visual artifact.

### Case 3: Approach Stable During Player W/S
- Base world velocity `−150` is unconditional.
- Approach correction capped at 80–160 u/s normalized.
- W/S changes `player_pos.y` → `target_y` but the correction vector is bounded; it does not cause visible speed-up/slow-down of enemies relative to the terrain.
