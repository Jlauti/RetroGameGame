# NB-FIX-002 Runtime Notes — Enemy Fire Loop Hotfix

**Ticket**: NB-FIX-002
**Agent**: Pekka Kone (agent2)
**Date**: 2026-03-07

## Regressions Fixed

### Fix 1: Player Has No Health → Hostile Projectiles Never Damage

**Root Cause**: `PlayerShip` was spawned without a `Health` component. The hostile projectile collision handler queries `Query<(Entity, &mut Health), With<PlayerShip>>`, which returned nothing.

**Fix**: Added `Health::new(100)` to the player spawn in `setup_nebula_bouncer`. The player entity now has 100 HP, and hostile projectile hits call `hp.damage(10)` against a real component.

### Fix 2: No Player Failure Path From Hostile Damage

**Root Cause**: No system checked whether the player's health had reached zero from hostile fire. The existing `advance_player_crash_sequence` only triggers from surface crashes via `PendingCrashResult`.

**Fix**: Added `check_player_hostile_death` system that monitors the player's `Health` each frame. When `hp.is_dead()`, it activates `PendingCrashResult` which feeds into the existing `advance_player_crash_sequence` → `GameState::Results` flow. Crash VFX are also spawned via `spawn_neon_vector_crash_burst`.

### Fix 3: Combat Tokens Not Released When Enemies Die

**Root Cause**: When an enemy holding a combat token was killed by player orbs in `handle_orb_collisions`, the entity was despawned immediately. The `combat_token_system` never had a chance to release the token, causing `active_tokens` to drift permanently upward.

**Fix**: `combat_token_system` now recounts live token-holders every frame before processing releases and acquisitions. This self-heals the counter whenever entities with active tokens are despawned by any code path.

### Fix 4: Hostile Projectile Cap Bypassed During Same-Frame Firing

**Root Cause**: `enemy_fire_system` captured `projectile_count` once at the top of the system. When multiple enemies fired in the same frame, each saw the same stale count, so the cap was effectively ignored.

**Fix**: Replaced with `existing_projectile_count + spawned_this_frame`. `spawned_this_frame` is a running counter incremented each time a projectile is spawned within the current system invocation.

### Fix 5: Telegraphing Was Timer-Only, Not Visible

**Root Cause**: The `Telegraphing` state existed only as a timer. No visual cue was spawned, making the pre-fire window invisible to the player.

**Fix**: When an enemy transitions to `Telegraphing`, a `TelegraphVisual` child entity is spawned — a stretched, bright red-orange additive quad pointing toward the player (aiming-laser glow). This entity is despawned when the state transitions to `Firing`. Parameters:
- Color: `srgb(1.0, 0.35, 0.15)` with 0.55 alpha
- Emissive multiplier: 18×
- AlphaMode: Add (bloom-friendly)
- Scale: 200×3 units (long thin laser line)
- Rotation: aimed at player position at spawn time

### Fix 6: Hostile Projectiles Missing CollisionEventsEnabled

**Root Cause**: `spawn_hostile_projectile` did not include `CollisionEventsEnabled`, which Avian2D requires to generate `CollisionStart` messages. This meant `handle_hostile_projectile_collisions` never received events.

**Fix**: Added `CollisionEventsEnabled` to the hostile projectile spawn bundle.

## BRP/MCP Validation Notes

### Case 1: Hostile Projectile Hit Affecting Player Health / State
- `handle_hostile_projectile_collisions` now logs `"Hostile projectile hit player! HP: {current}/{max}"` on every hit.
- Player Health decrements by 10 per hostile projectile impact.
- Query path is live: `Query<(Entity, &mut Health), With<PlayerShip>>` matches because `Health::new(100)` is on the player entity.

### Case 2: Player-Death / Failure Path From Hostile Fire
- `check_player_hostile_death` monitors player HP each frame.
- When HP reaches 0, it activates `PendingCrashResult`, which feeds into `advance_player_crash_sequence` → `GameState::Results`.
- VFX crash burst is spawned at origin.
- Information log: `"Player killed by hostile fire — triggering failure path"`.

### Case 3: Multi-Enemy Fire Respecting Simultaneous Hostile-Shot Cap
- `spawned_this_frame` counter tracks within-frame spawns.
- Cap check is `(existing_projectile_count + spawned_this_frame) < max_projectiles_on_screen`.
- With max 12, even if all 3 token-holders fire simultaneously, the cap holds.

### Case 4: Visible Pre-Fire Telegraph
- `TelegraphVisual` entity spawns as a bright additive child of the enemy.
- Duration matches `HostileFireConfig::telegraph_duration` (default 0.5s).
- Visual is a stretched red-orange laser line pointing toward the player.
- Despawned automatically when state transitions to `Firing`.
