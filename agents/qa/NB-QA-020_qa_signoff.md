# QA Signoff — NB-QA-020

- Date: 2026-02-25
- Verdict: **FAIL** (Conditional)

## Acceptance Commands Run:

| Command | Exit Code | Result |
|---------|-----------|--------|
| `cargo build --bin retro-game-game` | 0 | PASS |
| `cargo test --lib nebula_bouncer` | 0 | PASS |
| `cargo fmt -- --check` | 1 | FAIL (Topography.rs) |

## Scope Boundary Check: PASS
- No gameplay code changes detected.
- No new assets generated.

## Findings & Observation Notes:

### 1. Camera Transform Deviations
- **Yaw mismatch**: The contract (`NB-A1-006`) specifies a Yaw of **15.0°**. The implementation in `setup_nebula_bouncer` uses `cam_yaw = 15.0f32.to_radians()`, but the direction of rotation might need verification against the visual "behind ship" feel.
- **Pitch**: Correct at **-30°**.
- **Look-At Offset**: The contract specifies `[0.0, 0.0, 4.0]`. The code uses `cam_offset = Vec3::new(0.0, 4.0 * 128.0, 0.0)`. In a Y-up world, Y is forward. However, the game seems to use Y as the scrolling axis. If Z is depth/up, the offset might be on the wrong axis.
- **Orthographic Scale**: Correct at **15.0**.

### 2. Visual Layering (HITL Regression)
- **Ship Occlusion**: HITL tests reveal that the 3D player model is being rendered **behind** the 2D topography hexes or ground tiles. This is likely due to the camera's tilt causing the `Z` depth of the tilted model to fall behind the `Z=0` plane of the sprites.
- **Fix Attempt**: An attempt to fix this via camera projection and scale adjustments was initiated but postponed per user directive to report findings for another agent.

### 3. Topography Implementation Issues
- **Missing Data Layer Integration**: `topography.rs` is still using a `mock_height` function instead of the `generate_chunk_topography` data layer implemented in `NB-A3-003`.
- **Seed Mismatch**: `spawn_next_chunk` generates a `topo_seed` using `fold_seed(0xDEADC0DE_BAADF00D, chunk_y.to_bits())` instead of using the `ProcGenState.global_seed` and `chunks_spawned` as required for full determinism.
- **Visual Tiers**: Quantization correctly implements 4 tiers [0, 1, 2, 3]. Colors match the hex edge colors from `NB-A4-011`.
- **Formatting**: `topography.rs` fails `cargo fmt`, suggesting recent unformatted edits.

### 3. Readability & Gameplay
- **Deterministic Aiming**: Raycast logic in `orient_player_to_cursor` correctly accounts for the Z=0 plane and camera projection. 
- **Collision View**: Tier 3 obstacles include a dim internal glow (10% alpha Magenta) as requested to improve readability.

## Recommendations:
- Integrate `generate_chunk_topography` in `topography.rs`.
- Standardize on `ProcGenState` seeds for topography generation.
- Fix axis alignment for `cam_offset` to ensure the ship sits in the lower third.
- Run `cargo fmt` to clean up `topography.rs`.
