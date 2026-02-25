# Task Report: NB-A2-005 - Enemy Ship .glb Runtime Integration

## Objective
Integrate `AlienFighter.glb` into Nebula Bouncer enemy runtime rendering, replacing the 2D sprites for enemies (or at least one archetype) with the 3D model.

## Changes Implemented
1. **`specs/future/nebula_bouncer/asset_manifest.json`**:
   - Updated `enemy_model_default` to point to `sprites/future/nebula_bouncer/ship_models/AlienFighter.glb#Scene0`.
2. **`src/eras/era_future/nebula_bouncer/resources.rs`**:
   - Added `enemy_model_default: String` to the `NebulaAssetManifest` Resource.
   - Initialized `enemy_model_default` using the same fallback path in the `Default` implementation.
3. **`src/eras/era_future/nebula_bouncer/systems.rs`**:
   - In `spawn_chunk`, replaced the `Sprite` bundle attached to new enemies with a `SceneRoot` component.
   - Bound `SceneRoot` to load `asset_manifest.enemy_model_default`.
   - Replaced `enemy_tint` logic with a uniform `scale_factor` based on the previously defined `Sprite` bounds, allowing each `EnemyArchetype` (Scout, Interceptor, Heavy, Bulwark) to spawn with distinct sizes relative to the base model size.
   - Set the Z-axis rotation for the spawning `Transform` to 180 degrees (`PI`), ensuring enemies face "downward" toward the player.
   - Cleaned up the `enemy_tint` dead code.

## Verification
- **Compilation**: `cargo build --bin retro-game-game` succeeded without syntax errors.
- **Unit Tests**: `cargo test --lib nebula_bouncer` executed and passed all 22 tests.
- **Formatting**: Executed `cargo fmt` to apply stylistic consistency.

## Conclusion
The Enemy Ship model integration fits alongside the recently added Player Ship `.glb` integration without regression. The ECS logic effectively uses uniform scaling to preserve the sizes of various archetypes while pointing them in the correct game direction.

Task is complete according to the Definition of Done.
