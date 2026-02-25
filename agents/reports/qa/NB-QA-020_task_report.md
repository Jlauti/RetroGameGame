# Task Report: NB-QA-020 - Nebula Camera + Topography Wave QA Signoff

## Summary
Completed the QA validation for the Nebula Camera and Topography visual wave. While the technical foundation (building, testing) is solid, the implementation has significant deviations from the approved contracts in `NB-A1-006` and `NB-A3-003`. 

**Verdict: FAIL** due to missing integration of the deterministic data layer and camera offset discrepancies.

## Changes Verified

### Camera System
- **Contract Check**: Verified `Pitch: -30.0°` and `Distance: 15.0 Units`.
- **Issue**: `Look-At Offset` is implemented on the Y axis in world space, which may not align with the intended "forward" look-at if the coordinate system assumes Z is up/depth.
- **Accuracy**: Orthographic projection scale is correctly set to 15.0.

### Visual Layering (HITL Critical Finding)
- **Issue**: The player ship model is being occluded by 2D ground and topography graphics. This is a result of the 3D perspective camera tilt interacting with the 2D sprite plane at Z=0.
- **Status**: Reported as a regression. Reverted an internal attempt to fix this to maintain build stability.

### Topography System
- **Visuals**: Hex tier colors (Electric Purple, Neon Cyan, Hot Magenta) correctly match the `NB-A4-011` baseline.
- **Logic**: Quantization into 4 tiers is correctly implemented.
- **Issue**: The runtime is still using a mock hash function (`mock_height`) in `topography.rs` instead of fetching the pre-computed grid from the `ChunkTopography` data layer.
- **Determinism**: The generation seed uses `chunk_y` rather than the global state seed, which may lead to floating-point drift over long sessions.

### Regressions & Stability
- **Build/Test**: Project builds successfully and `nebula_bouncer` unit tests pass.
- **Input**: Cursor raycasting correctly handles the new camera angle for aiming.
- **Movement**: Scrolling and chunk spawning logic remains stable.

## Gap Analysis
| Requirement | Status | Note |
|-------------|--------|------|
| Deterministic Topography | PARTIAL | Data layer exists but is not used by the renderer. |
| Chase Camera Angle | PASS | Pitch and distance match contract. |
| Topography Visual Tiers | PASS | Colors and quantization aligned. |
| Aiming Consistency | PASS | Deterministic cursor mapping preserved. |

## Recommendations for Handoff
1. Refactor `spawn_chunk_topography` to accept and use `ChunkTopography` data.
2. Align `cam_offset` with the gameplay "forward" axis (Z or Y depending on viewport interpretation).
3. Apply `cargo fmt` to `src/eras/era_future/nebula_bouncer/topography.rs`.
