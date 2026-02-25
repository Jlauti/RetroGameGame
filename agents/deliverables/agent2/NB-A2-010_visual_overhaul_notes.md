# NB-A2-010 Visual Overhaul Notes

Agent: agent2 (Pekka Kone)
Date: 2026-02-25

## Constants Changed

### Camera (systems.rs)

| Constant | Before | After | Reason |
|---|---|---|---|
| `cam_distance` | `11.0 * 128.0` (1408) | `8.0 * 128.0` (1024) | Closer chase framing |
| `cam_pitch` | `-30.0°` | `-35.0°` | Steeper angle for better ground read |
| `camera_target` forward multiplier | `2.8 * 128.0` | `2.0 * 128.0` | Less forward offset → ship closer to center |
| `viewport_height` | `11.0 * 128.0` (1408) | `8.0 * 128.0` (1024) | Tighter viewport |

### Topography (topography.rs)

| Constant | Before | After | Reason |
|---|---|---|---|
| Tier 0 color | `srgba(0.22, 0.38, 0.98, 0.08)` | `srgba(0.18, 0.42, 1.0, 0.14)` | Boosted alpha, adjusted hue |
| Tier 1 color | `srgba(0.58, 0.34, 0.94, 0.14)` | `srgba(0.58, 0.28, 0.96, 0.22)` | Boosted alpha |
| Tier 2 color | `srgba(0.0, 1.0, 0.94, 0.22)` | `srgba(0.0, 1.0, 0.90, 0.32)` | Boosted alpha |
| Tier 3 color | `srgba(1.0, 0.22, 0.86, 0.30)` | `srgba(1.0, 0.18, 0.80, 0.42)` | Boosted alpha |
| Elevation multiplier | `0.55` | `0.30` | Gentler dips/mounds |
| Footprint base | `0.84` | `0.90` | Less variation |
| Footprint height scale | `0.14` | `0.08` | Less variation |
| Footprint clamp range | `0.80–0.98` | `0.88–0.98` | Tighter range |

### Mesh (resources.rs + systems.rs)

| Change | Before | After | Reason |
|---|---|---|---|
| Topography mesh | `quad_mesh` (Rectangle) | `hex_mesh` (RegularPolygon 6-sided) | True hex silhouettes |
| `NebulaMaterials` struct | No `hex_mesh` field | Added `hex_mesh: Handle<Mesh>` | Shared hex mesh handle |

## Scroll Lock

- Confirmed: `cam_yaw = 0.0` (was already 0.0)
- Confirmed: chunks scroll on Y axis only (`translation.y -= delta_y`)
- No diagonal drift in code — no changes needed

## Design Decisions

1. **RegularPolygon**: Used Bevy's built-in `RegularPolygon::new(0.5, 6)` instead of manual vertex construction. Cleaner, avoids private API imports, and produces a correct flat-top hexagon.

2. **Alpha range 0.14–0.42**: Roughly doubled from original 0.08–0.30. Terrain remains subordinate to ship/projectile layer but is now clearly readable at gameplay zoom.

3. **Elevation amplitude halved**: From 0.55× to 0.30× — prevents steep geometry that could occlude gameplay elements.

## HITL Verification Required

- Forward-only scroll read
- Closer chase framing feel
- Hex tile silhouette clarity
- Relief readability
- Neon tier differentiation vs gameplay element hierarchy
