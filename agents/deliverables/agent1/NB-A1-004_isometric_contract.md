# NB-A1-004: Nebula Isometric 2.5D Camera Contract

## 1. Camera Configuration
- **Mode**: Orthographic
- **Transform / Orientation**:
  - **Pitch**: -45 degrees (rotation around the local X-axis).
  - **Yaw**: 45 degrees (rotation around the global Y-axis).
  - **Roll**: 0 degrees.
- **Scale / Zoom Baseline**: The orthographic projection should be scaled such that the viewport consistently shows **20 world units vertically**, regardless of window aspect ratio.

## 2. Gameplay Plane definition & Axis Mapping
- **The Plane**: All critical gameplay logic (movement, collisions, physics) occurs strictly on the **XZ plane** (Y = 0).
- **Axis Mapping**:
  - **+X**: Right
  - **-X**: Left
  - **+Z**: Backward (Down screen)
  - **-Z**: Forward (Up screen)
  - **Y**: Up (Depth / Height layering)

## 3. Cursor-to-World Mapping Policy (Aiming)
- **Aiming**: Since the camera is angled at 45/45, direct screen-space to world-space translation is skewed. To aim accurately, the game must cast a **Ray from the camera** through the cursor's screen position, intersecting with the **Y = 0 plane**. The intersection point is the precise world target for the player's aiming reticle and projectiles.

## 4. Depth & Readability Policy
To maintain clear visual hierarchy and prevent overlapping artifacts in the isometric view, elements must adhere to strict Y-axis layering:
- **Floor / Background Terrain**: `Y < 0.0` (Typically Y = -1.0 to -0.1)
- **Entities (Player, Enemies, Walls)**: Pivot/Base at `Y = 0.0`. Their volume extends upwards.
- **Projectiles / Kinetic Orbs**: Float above entities at `Y = 0.5`. This ensures they are clearly visible over flat ships and low walls.
- **VFX / Overlays**: Floating text, explosions, and high-priority particle effects at `Y = 1.0` or higher to guarantee they render on top of all models and projectiles.

## Rationale
This 2.5D true isometric contract (45/45) ensures the "Dark Synthwave" 3D models can be appreciated from a flattering angle, rather than a direct top-down view. Fixing the gameplay to the XZ plane prevents the physics engine from introducing unpredictable bouncing behavior, while the defined depth policy keeps the visual hierarchy instantly readable during chaotic moments.
