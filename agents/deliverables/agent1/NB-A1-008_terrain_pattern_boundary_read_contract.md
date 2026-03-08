# Nebula Terrain Pattern & Boundary Read Contract
**Ticket**: NB-A1-008
**Author**: Aarne Tasapaino (Gameplay)
**Target**: Pekka Kone (Engine/Runtime)

## 1. The Playable Envelope
To ensure combat readability, the playfield is structured into distinct envelopes:
- **Core Lane (Center 60%)**: Mostly clear. Contains minimal, deliberate, traversable ground motifs (like low ridges or valleys). No hard crash blockers permitted here unless explicitly spawned as part of a high-value structured encounter (e.g., a central pillar base).
- **Shoulders (Remaining 40%, Split 20/20 Left/Right)**: This is where extrusions are allowed to "press the lane." Extrusions in this zone act as ricochet banks or cover but must never fully pinch off the core lane.
- **Out of Bounds (Beyond Shoulders)**: Strict boundary geometry. Extrusions here form the impassable horizon and provide background depth, but do not interact with the player physics directly.

## 2. Ground Pattern Motifs vs Randomness
Random noise is forbidden. The ground must be built from authored structural motifs:
- **Readable Lanes**: Clear, unbroken forward paths that intuitively guide the player's eye and movement.
- **Ridge Lines**: Continuous, raised hex sequences running parallel to or diagonally across the play axis to define structural boundaries or lanes.
- **Side Pockets**: Recessed areas in the shoulders that provide temporary safe zones or spawn points for specific enemy types.
- **Ricochet Banks**: Clean, flat-faced, steeply-angled surfaces placed intentionally in the shoulders to facilitate bank-shots.
- **Traversal-Safe Valleys**: Smooth, continuous dips in the terrain that offer clear flight paths and visual contrast to the action taking place horizontally.

**Randomness threshold**: Ensure at least 80% of any chunk consists of clear, structured motifs. Random hex-height variance (greebling) is confined to the remaining 20% and must be constrained to low-impact visual noise on out-of-bounds extrusions only.

## 3. Playable Extrusions & Pressure Rules
- **Pressing the Lane**: An extrusion is allowed into the shoulder zone (pressing the lane) only if it serves a clear gameplay purpose: providing a ricochet surface, breaking line of sight, or steering the player toward the core lane. It must not block progression.
- **Staying Out of Bounds**: Decorative extrusions or monolithic structures intended purely for visual scale and world-building must remain strictly out of bounds.

## 4. Boundary Walls: The Neon Cage
The boundary walls must no longer read as generic slabs. They must read as deliberate containment.
- **Visual/Emotional Direction**: Use a "neon wire / fence / cage" aesthetic. The boundary should feel like a high-energy perimeter enclosing the playfield, not a solid concrete wall. It is an arbitrary arena boundary maintained by the current faction.
- **Camera Readability**: The fence should be partially transparent/holographic to allow the out-of-bounds background extrusions to remain visible, providing depth while clearly communicating the edge of the playable area.
- **Soft Pressure**: The boundary implies danger but should act as a soft-pressure surface (pushing the player back inside) before escalating to severe damage upon sustained contact.

## 5. Integrating Extrusions and Motifs
Extrusion groups must feel authored rather than scattered.
- Extrusions should naturally grow out of the ends of ridge lines or flank side pockets.
- High structures should serve as the caps to valleys or the backdrops to ricochet banks, visually cementing the ground motifs with vertical presence.

## 6. Implementation Notes for Pekka
- **Spawn-Envelope Rules**: Update the chunk generator to respect the 60% Core / 40% Shoulder split. Restrict high-Z hex spawning to the shoulders and out-of-bounds regions.
- **Visual Layering**: The neon wire/cage shader needs a distinct rendering layer so it reads over the background terrain. Ensure the transparency does not create depth-sorting artifacts with the ground hexes.
- **Runtime Validation**: Add a validation step or debug view to verify that the core lane maintains a guaranteed minimum traversable width. Add telemetry or debug gizmos for the ricochet bank normals to ensure they point inward toward the playable space.
