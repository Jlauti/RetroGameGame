# NB-A4-008 Model Validation

## Overview
This document contains the baseline validation for the `.glb` models delivered for the Nebula Bouncer game:
- `TechFighter.glb` (Player)
- `AlienFighter.glb` (Enemy)

## Runtime Loadability
- Both models are valid `glTF` binaries.
- Both models contain a single scene with one mesh (`geometry_0`).
- The `cargo check --bin retro-game-game` command passed successfully after adding them to the `asset_manifest.json`, verifying they do not break the rust compilation pipeline.
- The `cargo fmt -- --check` also passed cleanly.

## Baselines & Assumptions
Based on a programmatic inspection of the `.glb` files using a python script to parse the `JSON` chunks:

### `TechFighter.glb` (Player)
- **Scenes**: 1 (Unnamed, nodes: [0])
- **Meshes**: 1 (`geometry_0`)
- **Nodes**: 
  - Node 0: `world`
  - Node 1: `geometry_0`
- **Transforms**: Neither translation, rotation, nor scale components are explicitly defined on the nodes in the JSON, meaning they default to identity transforms:
  - **Translation**: `[0.0, 0.0, 0.0]` (Origin)
  - **Rotation**: `[0.0, 0.0, 0.0, 1.0]` (Identity Quaternion)
  - **Scale**: `[1.0, 1.0, 1.0]`

### `AlienFighter.glb` (Enemy)
- **Scenes**: 1 (Unnamed, nodes: [0])
- **Meshes**: 1 (`geometry_0`)
- **Nodes**:
  - Node 0: `world`
  - Node 1: `geometry_0`
- **Transforms**: Default identity transforms identical to `TechFighter.glb`.

## Integration Guidance
- **Orientation**: Both models use default coordinate spaces. The design docs state models should be "Y-up, facing +Z". In Bevy (`Y-up, right-handed`), this means `-Z` is forward for cameras normally, but the models should be spawned as required by the runtime logic.
- **Scale**: Since default scale is `[1,1,1]`, the integration agent (Pekka) should adjust the Bevy `Transform.scale` component to match the tile size during runtime rendering if the models appear excessively large or small.
