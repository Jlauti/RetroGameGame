# NB-A2-013 Task Report — Enemy AI + Hostile Fire Runtime

**Ticket**: NB-A2-013
**Agent**: Pekka Kone (agent2)
**Status**: DONE
**Date**: 2026-03-07

## Summary

Implemented the first runtime pass for enemy combat AI and hostile return fire in Nebula Bouncer. Enemies now exhibit readable combat behavior via a five-state state machine (Idle → Positioning → Telegraphing → Firing → Cooldown), role-based movement (Blocker/Flanker/Sniper), a combat token system limiting simultaneous attackers to 3, and hostile projectiles that are visually distinct, dodgeable, and capped at 12 on screen.

## Changes Made

### `components.rs`
- Added `EnemyRole` enum (Blocker, Flanker, Sniper)
- Added `EnemyState` enum (Idle, Positioning, Telegraphing, Firing, Cooldown)
- Added `EnemyAI` component with role, state, timers, engagement distance, LOS tracking, combat token flag
- Added `HostileProjectile` marker component
- Added `HostileFireSource` component with burst count, burst timer, and shots-left tracking

### `resources.rs`
- Added `CombatTokenPool` resource (max_tokens: 3, active_tokens tracking)
- Added `HostileFireConfig` resource (max_projectiles_on_screen: 12, projectile_speed: 150, telegraph_duration: 0.5, attack_cooldown: 1.5)

### `systems.rs`
- Added `archetype_to_role()` and `default_ai_for_role()` helpers
- Updated `spawn_next_chunk` to attach `EnemyAI` and `HostileFireSource` to spawned enemies
- Added `enemy_ai_system` — LOS evaluation + state machine tick
- Added `combat_token_system` — token acquire/release respecting max-3 simultaneous attackers
- Added `enemy_movement_system` — role-based positioning (Blockers center, Flankers shoulders, Snipers backline)
- Added `enemy_fire_system` — telegraph → burst fire → cooldown cycle with projectile spawn
- Added `handle_hostile_projectile_collisions` — destroys hostile shots on surface contact, damages player on hit
- Added `spawn_hostile_projectile` — creates bright red-orange emissive kinematic projectile entities

### `mod.rs`
- Registered all new types for reflection (EnemyRole, EnemyState, EnemyAI, HostileProjectile, HostileFireSource, CombatTokenPool, HostileFireConfig)
- Initialized CombatTokenPool and HostileFireConfig resources
- Added all six new systems to the Update schedule under NebulaBouncer playing state

## Contract Compliance

All implementation decisions trace directly to the NB-A1-009 enemy AI and return-fire contract:

| Contract Rule | Implementation |
|--------------|----------------|
| Three combat roles (Blocker/Flanker/Sniper) | `EnemyRole` enum, role assignment in `archetype_to_role` |
| 5-state machine (Idle/Positioning/Telegraph/Firing/Cooldown) | `EnemyState` enum, `enemy_ai_system` |
| Min 0.5s telegraph | `HostileFireConfig::telegraph_duration = 0.5` |
| Min 1.5s cooldown between attacks | `HostileFireConfig::attack_cooldown = 1.5` |
| Max 3 simultaneous attackers | `CombatTokenPool::max_tokens = 3` |
| Max 12 hostile projectiles on screen | `HostileFireConfig::max_projectiles_on_screen = 12` |
| Enemy shots do NOT ricochet | `handle_hostile_projectile_collisions` destroys on surface contact |
| No off-screen/behind-player attacks | LOS check enforces forward 180° arc |
| Projectile speed < player movement speed | 150 < 300 |
| Distinct visual for hostile fire | Red-orange emissive (srgb 1.0, 0.25, 0.1) vs player's cyan/blue |

## Acceptance Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ Pass |
| `cargo build --bin retro-game-game` | ✅ Pass |
| `cargo test --lib nebula_bouncer` | ✅ Pass (31/31) |
| `cargo fmt -- --check` | ✅ Pass |

## Deliverables

- Runtime notes: `agents/deliverables/agent2/NB-A2-013_runtime_notes.md`
- Task report: `agents/reports/agent2/NB-A2-013_task_report.md`
