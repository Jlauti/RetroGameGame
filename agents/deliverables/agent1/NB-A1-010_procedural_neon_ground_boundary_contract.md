# Procedural Neon Ground & Boundary Combat Contract
**Ticket**: NB-A1-010
**Author**: Aarne Tasapaino (Gameplay)
**Targets**: Aino Kuvitus (Visual Direction), Pekka Kone (Engine/Runtime)

## 1. Direction Lock
- Nebula's battlefield loop is `Star Goose terrain`: forward combat happens through terrain-shaped lanes, ridge pressure, side pockets, and shoulder banks that make the ground matter every few seconds.
- Sparse distance-highrun is rejected for this pass because it weakens the approved pillar that ground and environment matter, flattens the blocker/flanker/sniper role split, and removes the need for readable ricochet/cover choices.
- Side boundaries are a `soft cage + banks` system. The cage contains and pressures the player, but it is not an active bounce rail and must not become the main source of movement tech.
- Density stays `mixed`. The arena alternates between combat pockets, relief lanes, and lane-pressure segments instead of reading as constant-max clutter.
- Environment direction is `procedural neon`, not modeled-env-first. Ground geometry, materials, and environmental VFX may stay abstract and systemic as long as they communicate gameplay roles clearly.
- Enemies remain the primary combo-feed. Breakable hazards are frequent support targets that preserve pressure, open space, and occasionally return health without becoming a separate resource economy.
- Ricochet stays bonus play. Shoulder banks and selected fixtures create optional bank-shot value, but direct fire into enemies remains the default answer.
- Ships and enemies remain human-made `.glb` assets. The battlefield itself does not wait on a modeled ground kit.

## 2. Why Star Goose Terrain Wins
- `Star Goose terrain` preserves continuous forward pressure while letting topography shape targeting, dodging, line-of-sight, and lane ownership.
- It supports the already approved combat roles: Blockers contest the core lane, Flankers use the shoulders and pockets, and Snipers gain readable cover positions.
- It gives ricochet a readable home on the shoulders instead of making bounce shots arbitrary.
- It allows procedural generation to produce authored-feeling combat spaces from motifs instead of requiring finished bespoke environment kits first.
- Sparse distance-highrun is not the current direction because it would turn the battlefield into mostly empty approach space with occasional interruptions, making environment interaction secondary instead of constant.

## 3. Ground Motif Roster
| Motif | Primary Gameplay Role | Placement Rule | Blocking / Breakability Rule |
| --- | --- | --- | --- |
| Traversal-safe valleys | Maintain a readable forward route through the battlefield | Core lane baseline | Traversal-safe for player movement; not a blocker; may be visually contoured but not filled with mandatory hazard clutter |
| Ridge lines | Break sightlines and create enemy cover/topography pressure | Core-lane edges and shoulder transitions | Structural, projectile-blocking topography; not breakable |
| Shoulder ricochet banks | Offer optional bank-shot surfaces and angled cover | Inner and mid shoulders | Structural, ricochet-enabled, not breakable |
| Side pockets | Create risky side detours and flanking spaces | Just inside the side cage, branching off shoulders | Mixed use: structural banks define the pocket, breakables may live inside |
| Breakable hazard clusters | Support combo continuity and quick space-clearing | Shoulder zones and pocket interiors only | Destructible support targets; never the main lane-defining structure |
| Rare hard-gate setpieces | Force occasional commitment checks and lane changes | Rarely in late segment transitions, never as constant clutter | Structural hard blockers; not breakable; used sparingly |

## 4. Spatial Rules
### Core lane
- The core lane is the default traversal and direct-fire space.
- Traversal-safe valleys and readable ridge pressure belong here.
- Enemies, especially Blockers, may contest this space directly.
- Breakable hazards do not own the core lane. If a breakable is visible from core space, it should sit on a shoulder edge or pocket mouth rather than replace the lane with object clutter.
- Rare hard-gate setpieces may pinch the lane, but only as exceptions that create a decision point, not as the normal texture of travel.

### Shoulders
- Shoulders are where the battlefield gets denser.
- Shoulder ricochet banks, ridge pressure, flanker movement space, and most breakable hazard clusters belong here.
- Breakable shoulder fixtures are the primary support-target family for combo maintenance. They should reward aggressive side play without making the shoulders safer than the core lane.
- Shoulders may narrow the readable lane and create oblique firing angles, but they must still leave an understandable recovery route back to the core lane.

### Side cage
- The side cage is containment and pressure, not a traversal trick surface.
- Gameplay on the cage itself is limited to boundary warning, spatial compression, and visual depth framing.
- Gameplay just inside the cage is where side pockets, cage-adjacent breakables, and shoulder-bank exits belong.
- The cage must make edge over-commitment feel dangerous, but the meaningful combat interaction happens one step inside it, not by scraping directly along the wall.

## 5. Breakable Hazard Families
| Family | Destructible | May Drop Health | Combat Purpose | Placement Rule |
| --- | --- | --- | --- | --- |
| Breakable shoulder fixtures | Yes | No | Clear local clutter, keep combo pressure alive, open bank-shot space | Shoulder banks and shoulder exits |
| Pocket hazard clusters | Yes | Yes, but only one health-bearing target per cluster | Reward risky pocket dives with space clearing and occasional recovery | Inside side pockets and pocket mouths |
| Cage-adjacent support targets | Yes | No | Give the player a last-second pressure release target near the edge without making the cage itself a scoring lane | Just inside the side cage |
| Ridge spines / bank anchors | No | No | Hold the lane shape, block shots, preserve authored terrain reads | Ridge lines and bank roots |
| Hard-gate setpieces | No | No | Rare commitment checks that redirect traversal | Late/rare gate moments only |

## 6. Breakable Hazard Quantity Contract
- Breakables must be frequent enough to keep the combo loop alive between enemy kills, but sparse enough that enemies remain the reason the player pushes forward.
- `Combat pocket`: 2 to 4 destructible hazards total, arranged as 1 to 2 clusters, with at most 1 health-bearing target across the whole pocket.
- `Relief lane`: 0 to 1 destructible hazard total, used only as light upkeep pressure and never as the space's main point of interest.
- `Lane-pressure segment`: 1 to 2 destructible hazards total, biased to one shoulder or one pocket mouth so the player reads a pressured side instead of uniform clutter.
- Destructible hazards are support-value only. They may sustain combo flow, clear a line, or occasionally return health, but they must not outnumber active enemies in the same combat pocket.
- No fuel, ammo, or separate resource drops are introduced. Health is the only allowed recovery drop in this contract.

## 7. Density Cadence
### Combat pocket
- Reads as the main engagement beat.
- Mixed enemies plus shoulder/pocket breakables create several short-term decisions at once.
- The core lane stays readable, but the shoulders become meaningfully active.

### Relief lane
- Reads as a reset and reorientation beat.
- Terrain still matters, but clutter drops enough that the player can recover position, reacquire targets, and see the next pocket forming.
- Relief is not emptiness; it is lower-demand traversal-safe space.

### Lane-pressure segment
- Reads as a directed squeeze, usually from one shoulder or one pocket side.
- The player should feel asked to shift lane ownership, not simply survive random clutter.
- This segment is where the side cage and shoulder banks most often collaborate to create lateral pressure without hard-rail behavior.

## 8. Boundary Behavior Contract
- The side cage applies soft pressure by making edge play visibly compressed and tactically worse over time, not by bouncing the ship back like an arcade rail.
- The cage itself should remain visually transparent enough that out-of-bounds depth still reads.
- Shoulder banks are the real combat surfaces near the edge: they create ricochet opportunities, cover, and flank routing.
- Breakable shoulder fixtures and cage-adjacent targets belong just inside the cage so the player has a risky support-target option before hitting maximum edge pressure.
- Direct cage scraping must not become optimal play. If the player wants value near the boundary, that value comes from banks, pockets, and breakables inside the edge band.

## 9. What Procedural Neon Means In Gameplay Terms
- `Procedural neon environment` means the combat-significant battlefield language is authored through system rules: valleys, ridges, banks, pockets, cage bands, breakable fixtures, emissive materials, and boundary VFX all communicate gameplay roles without requiring bespoke chapter-by-chapter ground meshes first.
- `Procedural` does not mean noisy or random-first. Motif placement must still read intentional and repeatable at the level of combat roles and density cadence.
- `Neon` means high-contrast emissive language that clearly separates traversal-safe ground, projectile-blocking topography, ricochet-enabled surfaces, and boundary pressure from ships, enemies, and hostile projectiles.
- Abstract/systemic pieces allowed to remain procedural in this phase:
  - ground slabs and contour fields
  - ridge and bank geometry
  - side cage geometry/material/VFX
  - breakable fixture geometry/material/VFX
  - impact, hit, and boundary pressure VFX tied to environment semantics
- Human-made 3D `.glb` actors still required in this phase:
  - player ship
  - enemy ships
  - any future elite/boss ship actors
- No modeled ground kit is required before runtime/procgen implementation can proceed.

## 10. Visual-Runtime Alignment Rules
- Gameplay readability outranks decorative variety. Any neon treatment that hides whether a surface is safe, blocking, breakable, or ricochet-enabled is invalid.
- Structural terrain must read heavier and more stable than breakable fixtures.
- Breakables must be visually distinct enough that players can spot support-value targets at combat speed without confusing them for enemies.
- The side cage must read as a containment field with depth visibility, while shoulder banks read as the real near-boundary combat geometry.
- Chapter-specific art passes may recolor or re-skin the procedural environment language later, but they must preserve the same gameplay-readable motif roles.

## 11. Implementation Notes For Aino
- Produce one visual-direction packet for `procedural neon terrain language` that defines how traversal-safe valleys, ridge lines, shoulder ricochet banks, side pockets, breakable fixtures, and the side cage differ in silhouette, emissive treatment, and motion language.
- Emphasize distinction between `structural terrain`, `breakable support target`, and `human-made ship actor`.
- Include motion-language notes for:
  - cage pressure pulses or scan-lines
  - ricochet-capable bank highlights
  - breakable fixture pre-break readability
  - health-bearing breakables versus non-health breakables
- Do not replace the procedural direction with bespoke modeled environment concepts. The packet should support abstract/systemic runtime geometry first.

## 12. Implementation Notes For Pekka
- Procgen must emit explicit motif tags for traversal-safe valley, ridge line, shoulder ricochet bank, side pocket, breakable cluster, and rare hard-gate setpiece.
- Runtime metadata must distinguish:
  - structural versus destructible environment pieces
  - health-bearing versus non-health-bearing breakables
  - ricochet-enabled versus non-ricochet surfaces
  - core-lane, shoulder, and cage-adjacent placement zones
- Validation should confirm:
  - the core lane stays readable through each cadence segment
  - breakables do not outnumber active enemies inside a combat pocket
  - relief lanes remain visibly lower density than combat pockets
  - cage-adjacent value sits inside the boundary band rather than on the cage surface itself
- Runtime review tooling should let the team inspect motif distribution, destructible counts by segment, and health-drop source counts without relying on art-complete assets.

## 13. Follow-On Ticket Recommendations
- **Aino follow-on ticket**: `Nebula Procedural Neon Terrain Visual Language Sheet`
  - Scope: concept packet for motif silhouettes, emissive hierarchy, cage motion language, and breakable-vs-structural readability rules that preserve this contract.
- **Pekka follow-on ticket**: `Nebula Procedural Neon Procgen Motif Metadata Pass`
  - Scope: runtime/procgen implementation of motif tagging, destructible hazard metadata, density-cadence validation hooks, and cage/shoulder placement validation based on this contract.
