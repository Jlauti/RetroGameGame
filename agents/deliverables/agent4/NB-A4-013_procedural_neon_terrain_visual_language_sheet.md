# Nebula Procedural Neon Terrain Visual Language Sheet
**Ticket**: NB-A4-013  
**Author**: Aino Kuvitus (Art And Music Director)  
**Governing Contract**: `NB-A1-010 Procedural Neon Ground And Boundary Combat Contract`

## 1. Direction Lock
- This sheet defines the visual grammar for Nebula's procedural neon battlefield so runtime implementation can ship readable terrain and boundary semantics before any final modeled ground kit exists.
- The battlefield stays `abstract/procedural-first`. It must not read like a placeholder for missing ship art; it must read like the approved environment language.
- Human-made `.glb` assets remain required for the player ship, enemy ships, and any future elite/boss ship actors.
- Environment geometry, environment materials, boundary presentation, breakable hazard presentation, and environment-hit VFX may remain procedural/systemic as long as gameplay roles stay readable from the third-person camera.

## 2. Core Separation Rule
Nebula must preserve an immediate visual split between `environment systems` and `ship actors`.

| Category | Silhouette Rule | Emissive Rule | Motion Rule | Read Requirement |
| --- | --- | --- | --- | --- |
| Structural terrain | Open, faceted, planar, lane-shaping forms anchored to the ground field | Broad, stable emissive bands with low change frequency | No free-flight drift; only systemic pulses or hit-react flashes | Must read as part of the battlefield shell, never as an actor |
| Breakable support targets | Attached or clustered fixtures that interrupt terrain rhythm | Higher local contrast than structural terrain, but on smaller surface area | Idle instability allowed; stronger pre-break response than structural terrain | Must read as destructible support value, not as enemies |
| Cage pressure surfaces | Thin containment planes, wireframes, scan bands, or field ribs with visible depth through them | High-energy boundary read with transparency preserved | Continuous pressure motion, not impact-object motion | Must read as containment, never as a ricochet wall or solid ridge |
| Player and enemy ships | Closed hero silhouettes with clear front/back and thrust intent | Localized actor emissives reserved for engines, weapon ports, telegraphs, and faction accents | Thrust-driven, directional, animated as intentional actors | Must always win the actor-read over environment decoration |
| Player and hostile projectiles | Compact, high-contrast streaks/orbs with clear travel vector | Highest short-duration luminance in the frame | Fast, directional, transient | Must remain legible over every environment state |

## 3. Motif Grammar

### Traversal-Safe Valleys
- **Silhouette / shape**: broad low-contour channels, shallow-sided, continuous forward read, no protrusions that visually claim collision priority.
- **Emissive treatment**: lowest-intensity environment banding in the combat space; enough contour to show routing, but never brighter than breakable targets or cage pressure.
- **Motion language**: mostly still. Allow only slow contour drift or faint directional flow to support forward travel, not attention-grabbing animation.
- **Gameplay readability rule**: this is the calmest readable terrain state. If the player cannot instantly identify it as the default safe route, the treatment is too loud.

### Ridge Lines
- **Silhouette / shape**: heavier crest forms, stepped or faceted rises, continuous enough to read as line-of-sight blockers instead of scattered debris.
- **Emissive treatment**: brighter edge definition than valleys, with stable contour tracing on upper lips and shadowed lower faces.
- **Motion language**: near-static. Only low-frequency structural pulse and brief impact confirmation are allowed.
- **Gameplay readability rule**: ridges must read as durable projectile-blocking topography, not as breakables or ricochet prompts.

### Shoulder Ricochet Banks
- **Silhouette / shape**: deliberate sloped planes or curved facets angled toward combat space; they should read as engineered rebound surfaces rather than random rubble.
- **Emissive treatment**: bank-facing edges carry the clearest ricochet cue in the environment, but only on the strike-capable plane, not the whole shoulder mass.
- **Motion language**: inactive until relevant. Use conditional sweep/highlight behavior that activates when bank angle becomes tactically relevant or when struck.
- **Gameplay readability rule**: banks must advertise `optional bank shot here` without overpowering the direct-fire read toward enemies.

### Side Pockets
- **Silhouette / shape**: carved recesses branching off the shoulder band, visibly enclosed by structure on at least two sides, with a readable mouth back to the main lane.
- **Emissive treatment**: slightly denser local contrast than the core lane so the pocket reads as risky space, but still subordinate to enemies and projectiles inside it.
- **Motion language**: localized ambient churn is allowed to imply pressure and risk, but the pocket must not flicker so heavily that target acquisition drops.
- **Gameplay readability rule**: the player must read `detour zone with risk/reward value`, not `hidden arena offscreen`.

### Breakable Hazard Clusters
- **Silhouette / shape**: grouped fixtures, pods, struts, crystalline ribs, or modular stacks attached to shoulders/pocket interiors instead of replacing the lane skeleton.
- **Emissive treatment**: smallest footprint, highest environment-local contrast. Breakability should come from concentrated highlight zones, seams, or fault lines rather than overall size.
- **Motion language**: visible unstable idle state, escalating pre-break stress, immediate impact acknowledgment, then fast abstract collapse.
- **Gameplay readability rule**: clusters must be obviously secondary to enemies. They are support targets, not setpiece landmarks.

### Side Cage Containment
- **Silhouette / shape**: thin vertical or slightly arced containment field language with repeated systemic intervals; must preserve background depth visibility.
- **Emissive treatment**: strongest persistent boundary signal in the environment, but transparent by design. Brightness should come from edges/scan bands, not opaque fill.
- **Motion language**: continuous boundary pressure pulses, scan-lines, or traveling charge bands that imply `leave this edge band` rather than `bounce here`.
- **Gameplay readability rule**: the cage is containment only. It must never read like a safe wall, solid cover slab, or primary ricochet surface.

## 4. Emissive Hierarchy And Contrast Rules
The screen must preserve the following semantic order from lowest sustained attention to highest transient attention:

1. traversal-safe valleys
2. structural ridges and non-ricochet terrain contours
3. shoulder ricochet planes when inactive
4. breakable support targets and active ricochet callouts
5. cage pressure surfaces
6. player and enemy actor telegraphs
7. player and hostile projectiles, muzzle events, and hit-confirm flashes

Additional contrast rules:
- Structural terrain uses wide, calm emissive areas. Breakables use tighter, sharper contrast islands.
- Cage pressure may be brighter than terrain, but it cannot use the same spatial density as hostile projectile language.
- Ship actors must not inherit the same edge logic as terrain. Their emissive zones stay localized and function-led.
- Player/enemy ships and projectiles must remain distinguishable even when crossing bank highlights, cage pulses, or breakable stress states.
- Health-bearing breakables may use one extra secondary cue beyond the base breakable language, but that cue must stay within the breakable family and must not resemble enemy telegraphing.

## 5. Motion-Language Rules

### Cage Pulses / Scan-Lines
- Motion travels along the boundary field, not outward like an explosion.
- Pulses should imply sustained pressure and system enforcement.
- Motion cadence must be steady enough to feel systemic, not random or decorative.
- The field may intensify near player over-commitment, but never collapse into a solid flashing wall.

### Ricochet-Capable Bank Highlights
- Highlights are conditional and plane-specific.
- Use short surface sweeps, angle catches, or hit-echo traces to mark the rebound plane.
- Do not animate the entire shoulder mass. Only the rebound-capable face should light up as a tactical read.
- After impact, bank feedback should decay quickly so the screen returns to enemy-first readability.

### Breakable Pre-Break States
- Breakables require a visible unstable idle state before damage: seam flicker, internal stress crawl, fragment vibration, or intermittent fault-line glow.
- Increased damage should tighten the motion and shorten the interval between stress events.
- Health-bearing breakables can add a restrained inner pulse or contained core read, but still obey the same destruction family.
- Pre-break language must not imply enemy charging or target tracking.

### Hit Reactions
- Structural hits: brief contour flash or localized shard spark, with no wobble suggesting imminent destruction.
- Breakable hits: directional crack propagation, fragment shedding, and stronger local brightness pop.
- Cage hits/pressure contact: field ripple or charge distortion that stays on the containment plane and does not mimic a physical wall impact.
- Ricochet hit feedback: fast line-trace or angle-confirm flash that communicates rebound without becoming a projectile substitute.

### Abstract / Fractal / Systemic Destruction
- Breakable destruction should resolve into geometric fracture, shard fans, dissolving linework, or collapsing energy ribs rather than naturalistic debris clouds.
- The event should read as `system breaks apart` instead of `organic explosion`.
- Destruction must be fast, directional, and clean enough that the combat space reopens immediately.
- Structural terrain and cage surfaces do not use the full breakable collapse language unless the contract explicitly marks them destructible in a later ticket.

## 6. Readability Rules From Gameplay Camera
- The forward route through the valley must remain readable at a glance from the third-person combat camera.
- Environment silhouettes should bias horizontal and forward-guiding reads; avoid dense vertical stacks that eclipse ships or telegraphs.
- The environment may frame the lane, but it must not occupy the same visual priority tier as actors in the center third of the screen.
- Side-pocket depth cues must remain readable without requiring the player to inspect the literal boundary edge.
- Cage transparency must preserve out-of-bounds depth and horizon separation.
- If a visual treatment makes it harder to answer `safe route`, `blocking ridge`, `optional ricochet bank`, `breakable support target`, or `boundary pressure`, that treatment is invalid.
- Relief lanes must visibly decompress. Combat pockets may become denser, but they still need a readable actor corridor through the middle of the frame.

## 7. What Stays Procedural / Systemic
These environment elements may remain procedural/systemic for the current pass:
- valley floor contours and contour materials
- ridge and bank geometry
- side-pocket shaping geometry
- cage geometry, materials, and pulse behavior
- breakable hazard geometry, materials, and destruction effects
- environment hit flashes, bank-hit callouts, and boundary pressure effects

These still require human-made `.glb` assets:
- player ship
- enemy ships
- future elite or boss ship actors

This sheet does not authorize replacing ship silhouettes, enemy silhouettes, or hero gameplay actors with procedural forms.

## 8. Runtime Material And Asset Gaps To Respect
- Runtime needs distinct material/state hooks for `structural terrain`, `ricochet bank`, `breakable support target`, `health-bearing breakable`, and `cage pressure surface`.
- Runtime also needs the ability to trigger stateful motion changes instead of only static color swaps:
  - inactive versus active ricochet highlight
  - intact versus stressed versus breaking hazard state
  - baseline versus intensified cage pressure state
- No final modeled environment kit is required before these reads can be validated.

## 9. Handoff Summary For Pekka
- Prioritize semantic readability over decorative variation.
- Keep the environment language abstract, faceted, and systemic so it never competes with ship actors for authorship.
- Treat motion as semantic signaling:
  - cage motion means containment pressure
  - bank motion means optional ricochet value
  - breakable motion means destructibility and damage state
- Preserve the emissive hierarchy so projectiles and combat telegraphs always remain the brightest and shortest-lived reads in the frame.
