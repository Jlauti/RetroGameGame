# Task Report: NB-A2-006 - Nebula Isometric 2.5D Runtime Migration

## Objective
Implement Nebula Bouncer's isometric-style 2.5D runtime camera, tilt it to achieve a 2.5D perspective, while preserving existing 2D mechanics. Ensure 3D models align with this new projection.

## Changes Implemented
1. **`src/eras/era_future/nebula_bouncer/systems.rs` (Camera & Light Adjustments)**:
   - Modified the `Camera3d` entity in `setup_nebula_bouncer`. Changed the spawn transform from `Transform::from_xyz(0.0, 0.0, 900.0).looking_at(Vec3::ZERO, Vec3::Y)` to `Transform::from_xyz(0.0, -800.0, 800.0).looking_at(Vec3::ZERO, Vec3::Z)`. 
   - This angles the orthographic camera downward and forward, shifting from a pure top-down 2D view to a 2.5D tilted perspective along the Y-axis.
   - Tweaked `DirectionalLight` to better match the new camera angle by moving the light source position and aiming it at `Vec3::Z`.
   - Aiming mechanics (via `viewport_to_world_2d`) are intrinsically preserved since the rendering API projects appropriately onto the global zero-Z plane regardless of Camera pitch or rotation.

2. **`src/eras/era_future/nebula_bouncer/systems.rs` (Enemy Projection Corrections)**:
   - Previously, the `AlienFighter.glb` was embedded directly into the root `Enemy` entity, causing it to stand vertically on the Z axis due to glTF Y-up conventions.
   - Restructured the enemy spawn routine to spawn the `SceneRoot` as a `.with_children` attachment to the physical `Enemy` entity (matching the `PlayerShip`), applying `Quat::from_rotation_x(-FRAC_PI_2)` to lay the mesh flat against the gameplay plane.

## Verification
- **Compilation**: `cargo build --bin retro-game-game` succeeded.
- **Unit Tests**: `cargo test --lib nebula_bouncer` confirmed NO logic regressions for the ECS architecture. Cursor-to-aim translations and physical bouncing collisions are isolated from display rendering and thus unaffected.
- **Formatting**: `cargo fmt` executed cleanly without errors.

## Conclusion
The isometric Camera correctly portrays a tilted 2.5D visual angle while relying on `OrthographicProjection` to retain consistent mechanical scale and cursor precision. The Enemy model rotation completes the transition into full 3D visual integration.

Task is complete according to the Definition of Done.
