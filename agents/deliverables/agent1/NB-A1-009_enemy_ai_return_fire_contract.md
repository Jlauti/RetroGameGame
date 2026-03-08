# Enemy Combat AI & Return-Fire Contract
**Ticket**: NB-A1-009
**Author**: Aarne Tasapaino (Gameplay)
**Target**: Pekka Kone (Engine/Runtime)

## 1. First-Wave Combat Roles
To prevent combat from devolving into chaos, we introduce three distinct, non-overlapping enemy roles for the first wave:
- **Blockers (Frontline Heavies)**: Slow-moving or stationary. They hold the core lane and use structured terrain (valleys/ridges) to force the player into the shoulders or to commit to heavy direct fire.
- **Flankers (Shoulder Skirmishers)**: Fast and evasive. They stay strictly in the shoulder zones, weaving around extrusions and ricochet banks. Their job is to pressure the player from the sides and punish resting in the shoulders.
- **Snipers (Backline Artillery)**: Stationary or extremely slow. Positioned far back, often relying on extrusions for cover. They telegraph single, high-damage shots.

## 2. Movement Intent & Arena Interaction
- **Awareness**: Omniscience is forbidden. Enemies react based on clear proximity or line-of-sight triggers.
- **Positioning**: 
  - Blockers hold the center.
  - Flankers match player altitude/depth but stay out of the direct center lane (offset horizontally). 
  - Enemies must *never* clip through extrusions or out-of-bounds walls. They must path-find or hold position when backed into terrain.
- **Retreat/Pause**: If an enemy loses line of sight (e.g., player hides behind a ricochet bank), it pauses fire and either holds position or paths to re-establish sightlines. Flankers will actively chase; Snipers will hold.

## 3. Aiming & Firing Rules
- **Telegraphing**: Every attack must have a clear visual and audio telegraph (e.g., a charging glow or aiming laser for Snipers, a burst pre-flash for Flankers) lasting at least 0.5s before firing.
- **Lead Policy**: 
  - Blockers: Direct line-of-sight firing (no lead). 
  - Flankers: Slight prediction, but heavily clamped so quick direction changes by the player always result in misses.
  - Snipers: Strong prediction, forcing the player to change velocity or direction during the telegraph phase.
- **Burst & Cooldown**: Firing is always in distinct bursts or single shots. Continuous "laser beams" or zero-cooldown spam is banned. Every enemy must have a clear cooldown window (minimum 1.5s) between attacks.

## 4. Hostile Projectile Expectations
- **Readability**: Hostile shots must be bright, high-contrast, and visually distinct from player projectiles and the boundary neon cage. 
- **Speed**: Shot speed must be tuned heavily toward "dodgeable." Player movement speed must always comfortably exceed normal enemy projectile speed.
- **Count Limits**: Absolute maximum of 12 enemy projectiles on screen at once to prevent bullet-hell clutter.
- **Ricochet Rule**: Enemy projectiles **do not ricochet**. To maintain readability, hostile fire that hits extrusions or terrain is immediately destroyed.

## 5. Player Pressure Policy & Fairness
- **Threat vs. Fairness**: Return fire is meant to force movement, not to deal guaranteed damage. If a player reacts to a telegraph and changes direction, they should successfully dodge.
- **Simultaneous Attackers**: Maximum of 3 enemies allowed to fire at the player at any given moment. A "combat token" system should regulate who gets to attack.
- **Failure Modes to Avoid**: Do not spawn enemies directly behind the player. All combat must happen in the forward 180-degree arc where the camera is focused. No off-screen attacks.

## 6. Implementation Notes for Pekka
- **State Machine**: Implement a clean state machine for enemies (Idle, Moving/Positioning, Telegraphing, Firing, Cooldown).
- **Telemetry & Tuning Hooks**: Expose parameters for telegraph duration, projectile speed, attack cooldowns, and the simultaneous attacker limit (token count).
- **Runtime Validation**: Add debug rendering for enemy line-of-sight rays and movement paths to verify they are respecting terrain and not cheating.
- **Integration**: Ensure enemy projectiles use the established collision layers correctly, destroying themselves on terrain and boundaries without triggering player-specific hit logic like ricochet.
