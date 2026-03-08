# NB-FIX-002 Task Report — Enemy Fire Loop Hotfix

**Ticket**: NB-FIX-002
**Agent**: Pekka Kone (agent2)
**Status**: DONE
**Date**: 2026-03-07

## Summary

Fixed all five reviewed regressions in Nebula's enemy AI and hostile-fire loop:

1. Player now has `Health::new(100)` so hostile projectiles can deal real damage.
2. New `check_player_hostile_death` system triggers the existing crash/results failure path when player HP reaches zero.
3. Combat token accounting self-heals by recounting live token-holders every frame, so tokens are correctly reclaimed when enemies die or despawn.
4. Hostile projectile cap now tracks per-frame spawns (`existing_projectile_count + spawned_this_frame`) to prevent same-frame multi-enemy cap bypass.
5. Enemy telegraph state spawns a visible `TelegraphVisual` aiming-laser child entity (bright red-orange additive glow) instead of being timer-only.

Bonus fix: Added `CollisionEventsEnabled` to hostile projectile spawn so Avian2D generates collision events.

## Changes Made

### `systems.rs`
- **Player spawn** (line ~912): Added `Health::new(100)` and split bundle into `spawn()` + `.insert()` to stay under Bevy's 15-element tuple limit.
- **handle_orb_collisions** (line ~1690): Added comment noting token reclaim is handled by self-healing recount.
- **TelegraphVisual** component: New marker for visible telegraph child entities.
- **combat_token_system**: Recounts live active tokens every frame before processing releases/acquisitions — self-heals when enemies despawn.
- **enemy_fire_system**: Tracks `spawned_this_frame` for correct per-frame cap; spawns `TelegraphVisual` on Positioning→Telegraphing transition; despawns it on Telegraphing→Firing transition.
- **handle_hostile_projectile_collisions**: Added `info!` logging on player hit for BRP validation.
- **check_player_hostile_death**: New system — monitors player health, activates `PendingCrashResult` on death.
- **spawn_hostile_projectile**: Added `CollisionEventsEnabled` to the spawn bundle.

### `mod.rs`
- Added `check_player_hostile_death` to the Update system schedule.

## Acceptance Gates

| Gate | Result |
|------|--------|
| `cargo check` | ✅ Pass (no errors) |
| `cargo build --bin retro-game-game` | ✅ Pass |
| `cargo test --lib nebula_bouncer` | ✅ Pass (31/31) |
| `cargo fmt -- --check` | ✅ Pass |

## Deliverables

- Runtime notes: `agents/deliverables/agent2/NB-FIX-002_runtime_notes.md`
- Task report: `agents/reports/agent2/NB-FIX-002_task_report.md`
