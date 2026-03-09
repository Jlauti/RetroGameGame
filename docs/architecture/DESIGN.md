# RetroGameGame — Current Product Design

## Current Product Mode

RetroGameGame still has a broader long-term multi-era vision, but the active product lane is `Nebula Bouncer`.

For current execution, treat hub, carousel, launcher, and cross-era framing as background backlog unless they directly block Nebula work.

## Nebula Bouncer

Nebula Bouncer is a modern 3D third-person roguelite space shooter inspired by *Star Goose* (1988).

The current design pillars are:

- readable third-person ship combat
- strong movement and aiming feel
- chapter-based progression with clear faction identity
- ground and environment that matter to combat and navigation
- upgrade-driven replayability

## Chapter Structure

Nebula should be planned as a sequence of chapters with distinct combat identity.

The initial faction baseline is:

- scrapper faction building ships from junk
- biomechanical alien faction
- sleek futuristic faction

Each chapter should define:

- which faction owns the space
- enemy families and battlefield roles
- ground identity and environmental language
- encounter intent
- required concept art handoff
- required music brief

## Specialist Ownership

- `Aarne Tasapaino`: gameplay loop, progression, encounter rules
- `Pekka Kone`: runtime implementation and Bevy integration
- `Aino Kuvitus`: concept art direction and chapter music briefs
- `Ilmari Maasto`: chapter structure, factions, and ground identity
- `Sanna Laatu`: release-only QA and HITL validation
- `Veikko Fiilis`: dormant polish specialist, activated only on purpose

## Active Architecture Rule

Design around Nebula's current implementation needs first. Avoid expanding process or architecture around inactive launcher/carousel work until that lane is explicitly reopened.

## Nebula Runtime Notes

`NB-A1-007` adds these architecture-facing constraints for the next Nebula runtime pass:

- Keep movement and aiming on a stable gameplay plane even when ship presentation uses `visual skim` to read terrain contour.
- Separate surface semantics so player-contact outcome and projectile ricochet are authorable independently. A single visual shape is no longer enough to imply both behaviors.
- Treat boundary roles as explicit gameplay data: traversal surface, soft pressure boundary, hard crash blocker, plus a separate ricochet-surface flag for projectile response.
- Preserve direct-fire readability in camera and collision response. Terrain-follow presentation must not distort cursor projection, shot origin, or target legibility.
- Support telemetry for surface contacts, hard-blocker crash sources, ricochet attempts, and ricochet conversion into hits, kills, or score so the contract can be tuned after implementation.
- Terrain generators must guarantee a clean core-lane envelope and constrain hazard-level extrusions to the flanking shoulder zones (`NB-A1-008`).
- Procedural generation should emit authored motif combinations (ridges, valleys, ricochet banks) rather than uniform per-cell noise to ensure combat readability (`NB-A1-008`).
- Boundary rendering must support transparent / neon cage visual layers that composite over out-of-bounds depth geometry without z-sorting issues (`NB-A1-008`).
- Enemy logic requires a token-based concurrency limit to guarantee a maximum of 3 simultaneous attackers on screen (`NB-A1-009`).
- Enemy aim and state machines must support explicit telegraphing phases and cooldown windows to meet the fairness and dodgeability contract (`NB-A1-009`).
- Hostile projectiles require independent collision responses from player shots, strictly terminating on terrain/boundaries without ricocheting (`NB-A1-009`).
- Procgen/runtime must preserve the `NB-A1-010` motif roster as explicit gameplay data: traversal-safe valleys, ridge lines, shoulder ricochet banks, side pockets, breakable hazard clusters, and rare hard-gate setpieces.
- Spatial metadata must distinguish `core lane`, `shoulder`, and `cage-adjacent` placements so breakables and cover stay in their approved combat roles (`NB-A1-010`).
- Environment entities need separate flags for `structural`, `destructible`, `health-bearing`, and `ricochet-enabled` so support targets cannot be confused with lane-defining terrain (`NB-A1-010`).
- Density cadence must be inspectable in runtime terms: combat pocket, relief lane, and lane-pressure segment should be measurable for motif mix and destructible counts (`NB-A1-010`).
- Boundary implementation must keep the side cage as transparent soft pressure while placing meaningful near-edge combat on the interior shoulder banks and cage-adjacent fixtures instead of the cage surface itself (`NB-A1-010`).
- Runtime validation should expose motif distribution, breakable counts per segment, and health-drop source counts so the contract can be checked before art-complete assets exist (`NB-A1-010`).
