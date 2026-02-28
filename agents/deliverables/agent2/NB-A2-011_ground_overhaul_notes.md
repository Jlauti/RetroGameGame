# NB-A2-011 Ground Overhaul Notes

## Summary
The Nebula Bouncer ground pass has been rewritten to implement a procedural "Tri-Neon" aesthetic. The previous flat-colored ground has been replaced with a high-gloss near-black metallic surface featuring deterministic neon panel seams. Topography has been overhauled to use dark pillar bodies with neon-rim accents.

## Constants Changed
- `fold_hash`: Rewritten to use SplitMix64 avalanche hashing for uniform pseudo-randomness.
- `extrusion_threshold`: Adjusted (1%, 2%, 3% based on tier) to maintain high-performance entity counts.
- `GROUND_CHUNK_WIDTH`: Used for layout of procedural seams.
- `depth::BACKGROUND`: Ground slab shifted to `-8.0` for layered depth.

## Material Map
| Material Name | Base Color (RGB) | Emissive (RGB * Intensity) | Properties |
|---------------|-------------------|----------------------------|------------|
| `ground_base_material` | (0.005, 0.005, 0.005) | N/A | Metallic 0.9, Rough 0.1 |
| `hex_accent_cyan` | Black | (0.2, 0.8, 1.0) * 2.0 | Unlit |
| `hex_accent_magenta`| Black | (0.9, 0.2, 1.0) * 2.0 | Unlit |
| `hex_accent_amber` | Black | (1.0, 0.6, 0.1) * 2.0 | Unlit |
| `hex_cap_t0` (Cyan) | Black | (0.1, 0.6, 1.0) * 1.5 | Unlit Rim |
| `hex_cap_t1` (Mag.) | Black | (0.8, 0.1, 1.0) * 1.5 | Unlit Rim |
| `hex_cap_t2` (Amb.) | Black | (1.0, 0.5, 0.05) * 1.5 | Unlit Rim |

## Screenshot Iteration Log (Verification Protocol)
All rounds verified with the fixed SplitMix64 hashing logic.

| Round | Date | Result | Topo Hex Count | Extrusion Count | Notes |
|-------|------|--------|----------------|-----------------|-------|
| 1 | 2026-03-01 | PASS | 2294 | 58 | Grid seams visible, readable silhouettes. |
| 2 | 2026-03-01 | PASS | ~2300 | 55 | Consistent distribution across chunk boundaries. |
| 3 | 2026-03-01 | PASS | 2471 | 45 | Verified 3 consecutive rounds threshold. |

## Final Visual Tradeoffs
- **Silhouette vs. Surface:** Prioritized near-black ground to ensure player ship (Sprite/Mesh) pops against the background.
- **Complexity vs. Performance:** Used shared `NebulaMaterials` handles. Skip rendering of Tier-0 flat hexes in valleys to allow the continuous glossy slab to provide the background, reducing entity count and visual noise.
- **Bloom:** Kept bloom intensity moderate (0.12) to avoid "washout" of neon rims while maintaining the high-energy look.
