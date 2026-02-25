# Hotfix Visual Pack: Neon-Hex Topography (NB-A4-012)

This hotfix addresses the visibility and occlusion issues discovered in `NB-QA-020` where the 3D player ship is occluded by dense 2D topography graphics at the new -30° camera pitch.

## 1. Hotfix-Safe Tier Intensity

To prevent screen flooding and preserve the readability of the 3D models behind 2D sprites, all topography base alpha values must be reduced by ~50%.

| Tier | Hex Edge Color | Old Alpha | **Hotfix Alpha** | Notes |
| :--- | :--- | :--- | :--- | :--- |
| **0** (Floor) | Electric Purple (`#9b59f0`) | 15% - 20% | **5% - 8%** | Barely visible wireframe. |
| **1** (Raised) | Electric Purple (`#9b59f0`) | 35% - 40% | **12% - 15%** | Subtle structure. |
| **2** (High) | Neon Cyan (`#00ffff`) | 45% - 50% | **20% - 25%** | Keeps the high ground visible but highly transparent. |
| **3** (Wall) | Hot Magenta (`#ff00ff`) | 65% - 75% | **35% - 40%** | Clearly an obstacle, but semi-transparent so occluded ships remain readable. |

---

## 2. Edge / Fill Balance

The core issue causing "density" is overly bright internal faces on the hex grids. 

- **Tiers 0, 1, and 2**: Must use **0% Fill Opacity**. Only render the 1px edge outlines.
- **Tier 3 (Walls)**: Reduce internal block fill from 10% to **3% opacity**. This maintains the illusion of a solid wall but prevents total occlusion of ships hiding behind it.
- **Distance Falloff**: Increase atmospheric fade from 50% to **80%** in the top 1/3 of the screen.

---

## 3. Contrast & Readability Rules

- **Bloom**: **Disable all bloom** on topography layers. Bloom must be strictly reserved for projectiles (Orbs) and Entities (Ships/Enemies).
- **Z-Sorting Awareness**: Because the camera tilt causes 3D models to dip below the Z=0 plane visually, topography materials must either be rendered as "Additive" (so ships behind them simply tint the terrain) or use a custom depth-test logic that forces entities to render over Tiers 0-2.
- **VFX Clarity**: Transient VFX (impact flashes, hit rings) must force a Z-index of `depth::PARTICLES` to ensure they punch through the dimmed topography.

---

## 4. Asset Recommendations
As this is primarily a shader/material parameter update for the rendering agent (`Pekka`), no replacement sprite assets are required. The current hex/grid textures are structurally fine; their runtime color/alpha multiplication just needs to use these hotfix values.
