# NB-A4-009 Isometric Visual Alignment Pass

## Overview
This document outlines the visual validation for the Nebula Bouncer isometric scene migration regarding the `TechFighter.glb` and `AlienFighter.glb` models.

## Current State Observations
The `Camera3d` in `src/eras/era_future/nebula_bouncer/systems.rs` (on the `develop` branch) has been updated by **NB-A2-006** to use a 2.5D tilted perspective along the Y-axis:
`Transform::from_xyz(0.0, -800.0, 800.0).looking_at(Vec3::ZERO, Vec3::Z)`

When models are spawned:
1. the player ship (`TechFighter.glb`) is spawned rotated by `Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)`.
2. the enemy ships (`AlienFighter.glb`) are spawned similarly rotated `Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)`.

## Visual Validation
With the new tilted camera angle, the `-FRAC_PI_2` rotation along the X-axis lays the ships perfectly flat along the zero-Z gameplay plane. This causes the camera to view the ships slightly from the top-down/rear (for the player) and top-down/front (for the enemies if rotated on Z). 

This angle looks correct and sells the 2.5D "scrolling shooter" aesthetic well. 

## Baselines
### Player Model (`TechFighter.glb`)
- **Scale**: `vec3::splat(MODEL_UNIT_TO_WORLD)` (currently 80.0) scales the model appropriately to the tile size without clipping or rendering issues.
- **Orientation**: The `Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)` alignment operates correctly for laying it flat on the isometric grid. However, because the glTF native forward vector maps to "backward" (-Y) after being laid flat, the `player_forward_offset_deg` in `sprite_orientation.json` must be set to `90.0` to compensate.

### Enemy Model (`AlienFighter.glb`)
- **Scale**: The base default scale (`80.0`) matches the player. Scaling archetype bounds on top of a `Transform::from_scale` works cleanly.
- **Orientation**: Same as the player ship. Lay flat using `Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)`. The base `Enemy` entity does NOT need the legacy `Quat::from_rotation_z(PI)` spawn assignment. Native orientation works perfectly as long as `enemy_forward_offset_deg` in `sprite_orientation.json` is set to `90.0`.

## Conclusion
The current programmatic offsets implemented during the model loading phases align perfectly with the newly introduced isometric 2.5D camera tilt. No further baseline modifications to scale or orientation offsets are required for the models to read correctly in-game.
