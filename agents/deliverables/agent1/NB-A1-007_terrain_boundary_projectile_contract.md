# NB-A1-007 Terrain, Boundary, and Projectile Contract

## Scope

This contract defines the next Nebula combat pass under the locked defaults:

- ground follow is `visual skim`
- only explicit `hard blockers` are run-ending crash surfaces
- ricochet is a `skillful bonus`, not the main combat solution

The goal is to make terrain, boundaries, and projectiles read as one combat space instead of separate scenery and hazard systems.

## 1. Ship-to-Ground Follow Contract

### Core rule

Nebula flight remains a direct-control game on a stable gameplay plane. `Visual skim` means the ship reads as reacting to terrain height and contour, but terrain does not take over steering, aiming, or hidden collision.

### What the player should see

- The ship visually rides low over the ground instead of floating at one constant relationship to the map.
- When the terrain rises, the ship visually lifts with it early enough that the player reads the contour before crossing it.
- When the terrain falls away, the ship visually settles back into the basin instead of hovering as if the ground were irrelevant.
- Broad canyon walls, banks, and shelves should read as surfaces the ship is skimming past, not flat wallpaper.

### What the player should feel

- The battlefield has flow and contour.
- The ship still obeys the player's input immediately.
- Terrain adds spatial pressure and lane identity without turning the ship into a terrain-physics vehicle.

### Control and readability constraints

- Movement authority stays with player input; terrain follow is presentation, not auto-pilot.
- Aiming remains resolved on the stable gameplay plane so the reticle, cursor projection, and shot origin do not wobble with skim presentation.
- The camera may acknowledge terrain flow, but it must preserve target readability and a stable forward read.
- Terrain visuals must never imply collision where no gameplay collision exists.
- Non-lethal terrain cannot cause accidental run loss through scrape contact, camera pitch, or projectile spawn distortion.

## 2. Boundary Taxonomy

Boundary behavior must be authored through explicit gameplay roles rather than inferred from art shape alone.

Each surface gets one player-contact role:

- `traversal surface`
- `soft pressure boundary`
- `hard crash blocker`

Projectile response is its own explicit role:

- `ricochet surface`

That means a surface can be traversable, soft-pressure, or hard-crash for the player, and separately may or may not be ricochet-enabled for projectiles.

### Traversal surface

- Purpose: give the player something to skim over or alongside so terrain contours matter visually.
- Player outcome: safe. Contact or overlap does not end the run.
- Combat role: lane identity, contour read, and visual grounding.
- Projectile outcome: absorb or ignore unless also tagged as a ricochet surface.

### Soft pressure boundary

- Purpose: define the play space edge or a lane-compression surface without turning every touch into failure.
- Player outcome: non-lethal. The player is redirected, squeezed back inward, or otherwise discouraged from staying there, but the run does not end.
- Combat role: keep the fight centered, create pressure, and shape approach angles.
- Projectile outcome: only ricochets if explicitly tagged as a ricochet surface.

### Hard crash blocker

- Purpose: create explicit fail states, commitment checks, and high-stakes obstacle identity.
- Player outcome: run-ending crash on contact.
- Combat role: force route choice and punish clear misreads.
- Readability rule: these must read as unmistakably solid and dangerous before contact.

### Ricochet surface

- Purpose: provide deliberate bank-shot opportunities.
- Player outcome: does not by itself determine lethality; player outcome still comes from the player-contact role.
- Projectile outcome: clean, readable bounce with enough retained value to matter.
- Placement rule: use where the player can plausibly plan a bank shot, not where ricochet is accidental noise.

## 3. Player Failure Rules

- Only explicit `hard crash blockers` end the run.
- `Traversal surfaces` do not end the run.
- `Soft pressure boundaries` do not end the run.
- Terrain contour alone never counts as a crash surface.
- If a surface can kill the player, it must already read as a hard blocker before contact.

## 4. Hex Extrusion Role Split

Hex extrusions now split into two gameplay uses instead of one generic pillar hazard.

### Extrusions that remain hard blockers

These stay run-ending on player contact:

- tall, isolated pillars placed in the player's committed travel lane
- pinch-point pillars that intentionally create a yes-or-no threading decision
- extrusion groups that visibly seal a route segment except for a clearly open lane
- authored wall-connected extrusion masses that read as solid structure, not rebound banks

These are for commitment, route pressure, and explicit punishment. They should not be the majority case.

### Extrusions that become ricochet-first geometry

These are non-lethal to the player and primarily exist to create bank-shot value:

- flank-side posts and banks that sit off the main direct-fire line
- staggered pairs that open a visible reflection angle into enemy space
- shallow-angle lane shapers that bend approach paths without closing them
- cover-like extrusion clusters that create pressure relief if the player uses a bank shot well

These should read as tactical geometry first and crash hazards second. In this contract they are not crash hazards.

### Placement rules

Extrusion placement is successful when it does all of the following:

- preserves at least one readable direct-fire path through the encounter
- creates a second, optional ricochet answer that is useful but not mandatory
- changes lane value or timing instead of just consuming empty space
- interacts with enemy positioning so the bounce opportunity solves pressure the player can already see

### What makes an extrusion layout interesting

- it asks the player to choose between the safer direct line and the higher-payoff bank line
- it creates a readable pressure pocket that a ricochet can break open
- it shapes movement timing by making one side temporarily stronger or weaker
- it gives enemies a reason to feel protected until the player changes angle

### What counts as disposable noise

- random lone pillars with no effect on enemy pressure or route choice
- symmetric clutter that narrows space without creating a decision
- ricochet surfaces placed where the player cannot see the exit line
- obstacle density that turns every encounter into generic dodging instead of deliberate lane play

## 5. Projectile Contract Update

Ricochet stays a `skillful bonus`. Direct fire remains the primary way the player solves combat.

### Primary combat rule

- A clear direct shot must remain the baseline answer in ordinary play.
- Ricochet should create bonus damage, bonus score, or pressure relief when the player notices and uses a good bank line.
- Encounters may reward ricochet, but they must not require ricochet to feel fair or complete.

### Bounce expectations

#### Against enemies

- Enemy contact should read as a direct hit first.
- Enemy bodies are not ricochet teaching surfaces.
- Enemy hits must not produce random or hard-to-predict deflections.

#### Against walls and boundaries

- Only surfaces explicitly authored as `ricochet surfaces` should reliably bounce shots.
- Hard crash walls that are not marked for ricochet should absorb the projectile instead of creating surprise bank shots.
- Soft boundaries may reflect shots only when they are intentionally built to teach or reward a bank angle.

#### Against hex extrusions

- Ricochet-first extrusions should bounce reliably and read cleanly.
- Hard-blocker extrusions should not be assumed to be good bank-shot tools.
- If a hard-blocker extrusion is also made ricochet-capable for a specific encounter, the bounce line must still be obvious and the hard-blocker danger must remain readable.

### Reward intent

Ricochet rewards may include:

- extra damage through an additional hit line
- score gain tied to deliberate bounce use
- pressure relief by reaching enemies protected by geometry

Ricochet reward should come from using geometry well, not from mindless wall spam.

### Failure cases the projectile system must avoid

- bounce outcomes that look random
- bank shots that require off-screen prediction
- enemy collisions that throw the projectile into unreadable side angles
- repeated wall loops that stall the projectile in place
- surfaces that look reflective but absorb, or look absorbent but reflect
- impact feedback that hides whether the shot bounced, hit, or died

### Readability expectations

- projectile speed must leave enough time to understand whether a bounce is possible before firing
- bounce exit direction should be inferable from surface angle and impact point
- impact feedback must distinguish direct hit, ricochet, and projectile death
- the player should be able to track the first bounce outcome at a glance, even if later bounces are less exact

## 6. Implementation Notes For Pekka

These notes define contract needs without prescribing the exact runtime solution.

- Separate `visual terrain follow` from gameplay-plane movement and aiming. The player's authoritative movement, cursor projection, and shot logic should remain stable even when the ship presentation skims terrain.
- Represent boundary semantics explicitly. Current `Wall` and `HexExtrusion` markers are not enough on their own for the new contract because player lethality and projectile ricochet must be authorable independently.
- Keep collision response readable. Non-lethal boundaries should visibly push, slide, or redirect rather than silently behave like hidden blockers.
- Keep crash surfaces explicit. Hard blockers should produce unmistakable contact response and should not be confused with ricochet-first geometry.
- Ricochet response should preserve a predictable angle story. Do not let the physics layer create noisy micro-bounces that the player cannot read.
- Preserve direct-fire readability. Camera motion, ship skim presentation, and bounce effects must not pull the reticle or projectile origin away from the stable gameplay plane.

### Telemetry needs

Runtime telemetry should be sufficient to answer these questions during follow-up tuning:

- how often players contact traversal, soft-pressure, and hard-crash surfaces
- which hard-blocker category is causing crashes
- how often shots hit ricochet surfaces
- how often ricochet leads to enemy hits, kills, or score gain
- whether ricochet surfaces are being ignored because their angles are unreadable

## 7. Handoff Summary

For the next runtime pass:

- terrain should visually ground the ship without stealing control
- only explicit hard blockers should end the run
- most extrusions should stop being generic crash-pillars and instead split into clear hard-blocker or ricochet-first uses
- direct fire stays primary
- ricochet becomes a readable bonus layer that rewards skill and encounter awareness
