# NB-A2-013 Runtime Notes — Enemy AI + Hostile Fire

**Ticket**: NB-A2-013
**Agent**: Pekka Kone (agent2)
**Date**: 2026-03-07

## What Was Implemented

### Enemy AI State Machine

Each enemy now runs through a five-state cycle:

| State | Description |
|-------|-------------|
| `Idle` | Default. No engagement. Transitions to `Positioning` when LOS acquired. |
| `Positioning` | Active engagement posture. Waits for a combat token and minimum positioning time (0.65s) before entering telegraph. |
| `Telegraphing` | Pre-fire window. Contract minimum: 0.5s. After telegraph duration, transitions to `Firing`. |
| `Firing` | Burst fire. Shot count depends on role (Blockers: 3, Flankers: 2, Snipers: 1). 0.18s between burst shots. |
| `Cooldown` | Post-fire recovery. Contract minimum: 1.5s. Returns to `Positioning` after cooldown. |

### Combat Roles

Enemies are assigned a role based on their archetype:

| Archetype | Assigned Role | Movement Behavior |
|-----------|---------------|-------------------|
| Scout, Interceptor | **Flanker** | Shoulder zones, matches player depth, speed 190 |
| Heavy | **Blocker** | Holds center lane, speed 70 |
| Bulwark | **Sniper** | Stays far back, minimal movement, speed 45 |

### Combat Token System

- Maximum 3 simultaneous attackers (resource: `CombatTokenPool`).
- Tokens are released when an enemy enters Idle, finishes Cooldown, or loses LOS.
- Tokens are acquired by enemies in `Positioning` state with active LOS.

### Hostile Projectiles

- Bright red-orange unlit emissive spheres (color: `srgb(1.0, 0.25, 0.1)`).
- Default speed: 150 units/s (well below player movement speed per contract).
- Max 12 hostile projectiles on screen at once (resource: `HostileFireConfig`).
- Enemy projectiles do **not** ricochet — destroyed immediately on terrain/surface contact.
- Player takes 10 damage on hostile projectile hit.

### LOS / Forward-Arc Rule

- LOS requires enemy to be within engagement distance AND in the forward 180° arc (enemy Y > player Y − 120).
- Prevents off-screen/behind-player attacks per contract rule.

## Tuning Hooks Exposed

All tuning values are exposed via Bevy resources registered for reflection:

- `HostileFireConfig`: `max_projectiles_on_screen`, `projectile_speed`, `telegraph_duration`, `attack_cooldown`
- `CombatTokenPool`: `max_tokens`, `active_tokens`
- `EnemyAI` component fields: `engagement_distance`, `preferred_horizontal_offset`, role/state

These can be inspected and modified at runtime via BRP/MCP.

## BRP/MCP Validation Notes

### Case 1: Enemy Movement/Spacing/Engagement Intent
- Enemies exhibit distinct movement patterns based on role.
- Blockers remain center-lane, Flankers offset to shoulders, Snipers hold back.
- All enemies respect terrain bounds via the existing `ChunkMember` scroll system.

### Case 2: Hostile Fire Telegraph and Projectile Travel
- Hostile projectile visuals are bright red-orange emissive (high-contrast vs cyan/blue player orbs and green neon boundary).
- Telegraph state provides minimum 0.5s warning before shots.
- Projectile speed (150) is well below player movement (300), making shots clearly dodgeable.

### Case 3: Multiple Active Enemies Without Unreadable Spam
- Combat token pool limits simultaneous attackers to 3.
- On-screen hostile projectile cap at 12 prevents bullet-hell clutter.
- Burst cooldown of 1.5s ensures clear attack windows between bursts.
