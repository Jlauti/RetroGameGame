# Nebula Chase-Camera + Topography Contract (NB-A1-006)

This contract defines the camera positioning and ground topography visualization for the next Nebula Bouncer visual milestone.

## 1. Camera Transform Contract

To achieve a "chase" feel while preserving depth readability, the camera will move from its current strict isometric position to a lower, tighter angle.

| Parameter | Current Value | **New Contract Value** | Rationale |
|-----------|---------------|------------------------|-----------|
| **Pitch** | -45.0° | **-30.0°** | Lower angle increases the "horizon" feel and ship presence. |
| **Yaw** | 45.0° | **15.0°** | Nearer to 0° (true behind) but maintains slight angle for 3D depth. |
| **Distance** | Variable | **15.0 Units** | Tightens framing to make the player ship appear larger/closer. |
| **Look-At Offset** | [0.0, 0.0, 0.0] | **[0.0, 0.0, 4.0]** | Offsets the camera target forward so the ship sits in the lower screen third. |

> [!IMPORTANT]
> The camera must remain **Orthographic**. Viewport size should be adjusted to ensure 15 world units are visible vertically.

---

## 2. Player Framing Policy

- **Vertical Position**: The player ship should consistently occupy the lower ~30% of the screen.
- **Horizontal Position**: Centered on X=0 by default.
- **Movement Bounds**: Player ship is restricted to the lower 50% of the visible viewport to maintain forward visibility of upcoming terrain.

---

## 3. Neon-Hex Topography Contract

The ground uses a tiered hex-grid system where elevation is communicated through discrete vertical steps and neon highlights.

### Elevation Tiers

| Tier | Elevation Range | Visual Description | Gameplay Role |
|------|-----------------|--------------------|---------------|
| **0** | `0.00 - 0.24` | Baseline / Floor | Standard movement area. |
| **1** | `0.25 - 0.49` | Raised Platform | Slight elevation, mostly cosmetic. |
| **2** | `0.50 - 0.74` | High Ground | High tier, shots can still pass over. |
| **3** | `0.75 - 1.00` | Wall / Obstacle | Blocking terrain for projectiles/ship. |

### Topography Logic
- **Mapping**: Data from the procgen layer (Ilmari/Agent 3) provides values from 0.0 to 1.0 per hex. 
- **Discrete Steps**: The rendering layer (Pekka/Agent 2) must quantize these values into the 4 tiers above.
- **Visual Cues**: 
    - Vertical faces between tiers should be dark metallic.
    - Top edges of each tier MUST have a neon glow (Cyan for player-side/safe, Magenta for hazard-side/deadly).

---

## 4. Readability & Input Compatibility

- **Aiming**: Cursor raycast must still map accurately to the `Y=0` gameplay plane. The camera shift must not induce parallax errors in aiming logic.
- **Collision View**: Obstacles (Tier 3) must be silhouette-readable to prevent players from crashing due to perspective compression.
- **Settings Overlay**: The `Esc` menu should dim the active topography but maintain the background's neon glow for atmospheric consistency.
