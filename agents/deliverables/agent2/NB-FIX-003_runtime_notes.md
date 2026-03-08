# NB-FIX-003 Runtime Notes — Enemy Approach Direction and Facing Hotfix

**Ticket**: NB-FIX-003
**Agent**: Pekka Kone (agent2)
**Date**: 2026-03-08

## Root Cause Analysis

Two tightly connected bugs caused enemies to read as convoy traffic instead of incoming threats:

### Bug 1: Enemies Were ChunkMember

Enemies were tagged with `ChunkMember`, which caused `update_level_scrolling` to apply the terrain scroll delta (`-150 * dt` per tick on Y) to every enemy along with all floor tiles, walls, and topography. Since enemies also had `RigidBody::Dynamic`, the physics engine handled their transform but the scroll system fought the physics body via direct `transform.translation.y -= delta_y` writes. The result: enemies translated at the same rate as the scenery, appearing as same-lane convoy traffic relative to the player rather than as incoming threats moving against the scroll.

### Bug 2: Enemy Model Facing Was Identical to Player

The enemy model rotation was `Quat::from_rotation_z(PI) * Quat::from_rotation_x(FRAC_PI_2)` — the exact same transform applied to the player ship. This tilts a Y-up model into the XY gameplay plane and spins it to face `+Y` (forward, in the direction the player moves). Enemies facing `+Y` look like they are flying *away* from the player or in the same direction — not approaching.

### Bug 3: Movement Targets Were Scroll-Relative, Not Absolute

`enemy_movement_system` computed target positions as `player_pos.y + offset` and tried to reach them via direct transform mutation. Since the scroll system was simultaneously mutating the same transforms, the effective velocity was `approach_speed - scroll_speed (150)`, which for Blockers (70 units/s) meant they actually drifted backward with the scroll instead of holding or approaching. The AI appeared to function but produced no real forward pressure.

## Fixes Applied

### Fix 1: Removed ChunkMember from Enemy Spawn

Enemies no longer carry `ChunkMember`. They are now permanent world-space entities that locomote independently and are culled by a dedicated system.

### Fix 2: Fixed Enemy Model Facing

Changed enemy model rotation from:
```rust
Quat::from_rotation_z(PI) * Quat::from_rotation_x(FRAC_PI_2)
```
to:
```rust
Quat::from_rotation_x(FRAC_PI_2)
```

The `X(FRAC_PI_2)` tilt brings the Y-up glTF model into the XY gameplay plane. Omitting the `Z(PI)` spin means the model faces `-Y` (toward the player), which reads correctly as an oncoming threat.

### Fix 3: Overhauled enemy_movement_system

The new movement model:
- Computes absolute world-space target positions ahead of the player (`player_pos.y + role_offset`) — always in the forward combat arc.
- Drives `LinearVelocity` directly (physics-integrated), so Avian2D handles the actual movement rather than direct transform writes that conflicted with the scroll.
- Approach speeds increased to overcome the 150 units/s scroll: Blockers 160, Flankers 260, Snipers 90. At Sniper speed (90 < scroll 150), Snipers drift back slightly during approach but hold position once at their deep engagement Y.
- Uses velocity lerp for smooth deceleration at target.
- Brakes when at target depth.

### Fix 4: Added cull_behind_player_enemies

New system that despawns enemies that have drifted more than 800 units behind the player Y position. Returns combat tokens when culling token-holding enemies.

## BRP/MCP Validation Notes

### Case 1: Enemies in Forward Combat Arc
- Enemies spawn at `chunk_y + spawn.position.y` which is always ahead of the player (`CHUNK_PREFETCH_LEAD_Y = 3000`).
- `enemy_movement_system` drives them to hold at `player_pos.y + 290–620` depending on role.
- `enemy_ai_system` LOS check: `enemy_pos.y > player_pos.y - 120` — enemies must be in the forward 180° arc to engage. This check still works because enemies are now held ahead of the player.

### Case 2: Enemy Orientation / Facing
- Enemy model rotation is `Quat::from_rotation_x(FRAC_PI_2)` — tilted into gameplay plane, facing -Y (toward player).
- Player model rotation is `Quat::from_rotation_z(PI) * Quat::from_rotation_x(FRAC_PI_2)` — facing +Y (forward).
- The two are visually opposed, making enemy "incoming" body language clear.

### Case 3: Incoming Motion Against Scroll
- Enemies drive velocity at 160–260 units/s toward their engagement Y, against the 150 units/s terrain scroll.
- Net: Blocker has +10 units/s forward pressure, Flanker has +110 units/s. They visibly approach and hold position in front of the player.
- Previously: net drift was negative for Blockers (−80 units/s), making them scroll backward like terrain.
