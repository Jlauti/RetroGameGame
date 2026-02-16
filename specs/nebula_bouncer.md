# Nebula Bouncer (Working Title)

## Concept
A **Top-Down Scrolling Roguelite Shooter** that blends the terrain-navigation of *Star Goose* with the physics-based combat of a brick-breaker and the build-crafting of a modern roguelite.

**Core Hook**: Your bullets don't just disappear—they **bounce**. Mastery of angles and geometry is as important as reflexes.

## Aesthetic & Era
- **Proposed Era**: Era 3 (2000s) or Era 4 (2010s) — "The Indie Renaissance"
- **Visual Style**:
    - Neon/Vector graphics (Geometry Wars style) or crisp 32-bit pixel art.
    - High contrast to make projectiles and bounce trajectories visible.
    - **Juice**: Screen shake on heavy impacts, particle trails for bouncing bullets.

## Controls
- **Movement**: `WASD` / `Arrow Keys` — Control ship position relative to the scrolling screen.
- **Aiming**: `Mouse` — Controls a targeting reticle.
- **Fire**: `Left Click` — Launch Kinetic Orbs (bullets).
- **Skill**: `Right Click` / `Space` — Active ship skill (Dash, Shield, etc.).

## Core Mechanics

### 1. The Kinetic Orb (Weapon)
Unlike standard shmups where bullets fly straight:
- **Bouncing**: Basic projectiles bounce off terrain and screen edges 1-3 times before expiring.
- **Skill Shots**: Damage increases with each bounce (Ricochet Bonus).
- **Physicality**: Projectiles have mass; they can push small debris or be deflected by shielded enemies.

### 2. Terrain & Scrolling
- Vertical scrolling (like *Star Goose*).
- **Narrow Passages**: Terrain isn't just an obstacle; it's a **weapon**. Player shoots effectively *into* walls to bounce shots around corners or hit shielded enemies from behind.
- **Hazards**: Bumper walls (add velocity), Dampener fields (slow shots), Prisms (split shots).

### 3. Roguelike Evolution (The Build)
Enemies drop **Essence** (XP). Leveling up offers a choice of 3 random upgrades:

#### Weapon Elements (The "What")
- **Plasma**: Standard damage, medium speed.
- **Cryo**: Slows enemies on hit.
- **Tesla**: Chains lightning to nearby foes on impact.
- **Void**: Gravitational pull, damaging aura.

#### Weapon Modifiers (The "How")
- **Elasticity**: Increases max bounces.
- **Splinter**: Projectile splits into 2 smaller ones on first bounce.
- **Mass**: Projectile is slower but larger, pierces enemies, pushes them back.
- **Velocity**: Faster shots (harder to aim, higher DPS).

#### Synergies (The "Combo")
Combining specific elements and modifiers unlocks **Evolved Weapons**:
- *Tesla* + *Splinter* = **Storm Cloud**: On bounce, releases a nova of lightning bolts.
- *Cryo* + *Mass* = **Avalanche**: A giant snowball that rolls over enemies, growing larger.
- *Plasma* + *Elasticity* = **Photon Pinball**: Extremely fast, high-bounce laser that lasts 10s.

### 4. Ship Evolution (RPG Element)
Every boss defeat allows a permanent **Hull Evolution** for the run:
- **Interceptor Class**: High speed, Fire rate bonus, low HP.
- **Frigate Class**: High HP, Shield regen, slower.
- **Cruiser Class**: Dual weapon slots, massive hitbox.

## Game Loop
1.  **Launch**: Start run with basic kit (e.g., standard Orb).
2.  **Sector 1**: Navigate terrain, bounce shots to kill bugs/ships.
3.  **Level Up**: Draft cards/upgrades.
4.  **Boss**: Multi-stage fight requiring precise bounce angles.
5.  **Evolution**: Evolve ship hull.
6.  **Loop**: Harder sectors, more complex terrain.
7.  **Death**: Unlock new starting pilots/loadouts based on achievements.

## Technical Requirements (Bevy)
- **Physics**: Simple AABB/Circle cast for performance, custom reflection logic.
- **Inputs**: Mouse to World coordinate mapping.
- **ProcGen**: Since it's a scroller, terrain needs to be assembled from chunks to ensure "bounceable" geometry.
