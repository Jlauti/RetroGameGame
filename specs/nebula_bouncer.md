# Nebula Bouncer

## Concept

Nebula Bouncer is a modern 3D third-person roguelite space shooter inspired by *Star Goose* (1988).

The goal is not to recreate the original literally. The goal is to capture its forward-driving space-combat spirit and reinterpret it as a modern, chapter-based roguelite with stronger movement, stronger combat readability, and stronger faction identity.

## Current Design Pillars

- third-person ship combat that feels deliberate and readable
- a battlefield where ground and environment matter
- chapters with distinct faction and terrain identity
- progression that supports replayability without diluting combat clarity
- visual and audio direction that support a coherent chapter mood

## Active Product Direction

Nebula is the active product lane for this repository.

Carousel, launcher, and broader collection framing are background backlog unless they directly block Nebula work.

## Chapter Planning Baseline

Current chapter planning starts from three factions:

1. scrapper faction building ships from junk
2. biomechanical alien faction
3. sleek futuristic faction

Each chapter brief should define:

- faction owner
- enemy families
- ground identity
- encounter intent
- art concept needs
- music brief

## Implementation Priorities

- strengthen the core playable combat loop after the completed 3D migration
- support third-person readability in camera, movement, spawning, and encounters
- turn chapter planning into implementation-ready faction and enemy direction
- use 2D concept planning to support later 3D asset creation

## Directional Constraints

- final 3D gameplay assets are human-created
- art direction work should support later 3D modeling, not replace it
- QA is release-only by default, not part of every ticket
- specialist agents should only read the context directly relevant to their role

## Approved Terrain And Ricochet Contract

`NB-A1-007` establishes Nebula's next combat-pass rules for terrain, boundaries, and projectile behavior.

- Ground follow is `visual skim`: the ship should read as following terrain contours, but player movement, aiming, and projectile origin stay on a stable gameplay plane.
- Only explicit `hard crash blockers` end the run. Terrain contour, traversal surfaces, and soft pressure boundaries do not.
- Boundary semantics are split into four authored roles: traversal surface, soft pressure boundary, hard crash blocker, and ricochet surface.
- Hex extrusions are no longer one generic hazard type. Some remain hard blockers for commitment checks; others become ricochet-first geometry that shapes lanes and creates optional bank-shot value.
- Direct fire remains the primary combat answer. Ricochet is a readable, skillful bonus that can add damage, score, or pressure relief without becoming mandatory encounter tech.

## Approved Terrain Pattern And Boundary Read Contract

`NB-A1-008` establishes the structural rules for ground motifs, extrusions, and boundary presentation.

- Playable space is divided into a 60% minimal-hazard Core Lane and 40% flanking Shoulders where extrusions press the lane to create ricochet banks and cover.
- Terrain generation must prioritize structured motifs (lanes, ridges, side pockets, ricochet banks, and traversal-safe valleys) over random noise. Randomness is bounded to minor visual greebling on out-of-bounds geometry.
- Boundary walls use a "neon wire/cage" aesthetic, providing a transparent, high-energy containment surface that allows background depth to read while exerting a soft push-back on the player.
- Generative structures (extrusions) are anchored to ground terrain patterns so the space feels authored, connected, and deliberate.

## Approved Enemy Combat AI And Return-Fire Contract

`NB-A1-009` defines the core enemy behavior and projectile rules so enemies exert readable, fair pressure.

- Enemies occupy distinct combat roles: Blockers (core lane space control), Flankers (shoulder harassment), and Snipers (backline burst damage).
- Aiming and firing prioritize readability: all shots must be telegraphed, firing occurs in distinct bursts with cooldowns, and continuous/omniscient tracking is forbidden.
- Hostile projectiles are highly visible, dodgeable, and strictly destroyed on contact with terrain or boundaries (no enemy ricochets).
- Combat maintains fairness through a simultaneous attacker limit (max 3 at once) and restricting engagements to the player's forward-facing 180-degree camera arc.

## Approved Procedural Neon Ground And Boundary Combat Contract

`NB-A1-010` locks Nebula's next battlefield pass around `Star Goose terrain` rather than sparse distance-highrun.

- The arena remains structured around traversal-safe valleys, ridge lines, shoulder ricochet banks, side pockets, breakable hazard clusters, and rare hard-gate setpieces.
- Side boundaries are a `soft cage + banks` system: the cage contains and pressures the player, while meaningful combat interaction near the edge happens on shoulder banks and cage-adjacent fixtures just inside the boundary.
- Density is intentionally mixed across combat pockets, relief lanes, and lane-pressure segments instead of staying at constant-max clutter.
- Enemies remain the primary combo-feed. Breakable hazards are frequent support targets that clear space, sustain pressure, and only occasionally drop health; no fuel/resource economy is introduced.
- `Procedural neon environment` means abstract/systemic ground geometry, materials, and VFX may carry the active environment language, while ships and enemies remain human-made `.glb` assets.
