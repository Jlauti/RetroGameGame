# Visual Baseline: Neon-Hex Topography (NB-A4-011)

This document establishes the visual standards for the **Neon-Hex Topography** system in **Nebula Bouncer**, ensuring aesthetic consistency and gameplay readability at the new **-30° Pitch** camera angle.

## 1. Topography Tier Palette

Elevation is communicated via hex edge colors and intensity. Vertical faces are always **Metallic Charcoal** (`#1a1a24`).

| Tier | Mapping | Hex Edge Color | Intensity (Alpha) | Rationale |
| :--- | :--- | :--- | :--- | :--- |
| **0** | `0.00 - 0.24` | **Electric Purple** (`#9b59f0`) | 15% - 20% | Subtle baseline floor. |
| **1** | `0.25 - 0.49` | **Electric Purple** (`#9b59f0`) | 35% - 40% | Elevated but safe. |
| **2** | `0.50 - 0.74` | **Neon Cyan** (`#00ffff`) | 45% - 50% | High ground / tactical. |
| **3** | `0.75 - 1.00` | **Hot Magenta** (`#ff00ff`) | 65% - 75% | Obstacle / Hazard. |

---

## 2. Readability Guardrails

To prevent the "Neon Washout" effect (where entities disappear against terrain), implementation must follow these rules:

### A. The 50% Intensity Rule
- **Ground Hexes**: Must never exceed **50%** of the maximum brightness (HDR intensity) of the Player/Enemy models and Projectiles.
- **Fading**: Hex intensity should fade by 50% in the top 1/3 of the screen (distance falloff) to keep focus on immediate gameplay.

### B. Line Weight Separation
- **Terrain**: 1px crisp lines at native resolution.
- **Entities**: 3px - 5px soft-glow outlines. The thickness difference ensures ships read as "objects" and hexes read as "surface."

### C. Color Overlap Policy
- **Hazard Orange Entities**: When an orange enemy is on Tier 3 (Magenta) terrain, the enemy should gain a **Neon Cyan** rim light to maintain silhouette separation.

---

## 3. Tuning Recommendations

- **Bloom**: Apply bloom primarily to entities. Terrain hexes should have minimal to zero bloom to prevent "eye fatigue."
- **Noise**: Add a subtle "circuit hum" flicker (±5% opacity) to Tier 2 and Tier 3 hexes to make the world feel alive but distinct from static ships.
- **Glowing Core**: Tier 3 obstacles should have a very dim internal glow (10% opacity) in the center of the hex to make them read as solid pillars rather than just wireframes.

---

## 4. Reusable Assets & Callouts

- **Required**: A single "Hex Outline" sprite or a shader that can generate tiling hexes with variable edge thickness and pulse parameters.
- **CTO Callout**: Final confirmation needed on whether Tier 2 (Cyan) is "Safe" or just "High."
