# NB-A2-011 Runtime Notes

## Runtime Setup

- Binary: `target/debug/retro-game-game.exe`
- Env:
  - `RETRO_DEV_BOOT=nebula`
  - `BEVY_BRP_ENABLE=1`
  - `BEVY_BRP_PORT=15702`
- BRP methods used:
  - `world.query`
  - `world.get_resources`
  - `world.insert_resources`
  - `world.insert_components`
  - `world.mutate_components`
  - `brp_extras/send_keys`
  - `brp_extras/screenshot`

## Captured Cases

### Terrain Follow + Direct Control

- BRP key injection: `KeyD` for `350 ms`
- Player transform changed from `[17.2254, -1208.7845, 38.0]` to `[126.0199, -1245.0757, 38.0]`
- Telemetry after movement:
  - `terrain_follow_samples = 4013`
  - `max_skim_height = 0.2508809`
- Artifact: `agents/deliverables/agent2/NB-A2-011_terrain_follow.png`

### Direct-Fire Baseline

- Cursor-driven BRP shooting was not reliable enough to isolate a non-ricochet lane.
- Deterministic proof used the live orb pool:
  - reactivated pooled orb entity `4294963823`
  - set `ricochet_count = 0`
  - placed the orb on a live enemy position through BRP component mutation
- Telemetry result:
  - `direct_enemy_hits = 1`
  - `ricochet_surface_hits = 0`
  - `ricochet_enemy_hits = 0`
  - `last_projectile_event = DirectHit`

### Controlled Ricochet

- BRP validation-command shot captured a real bounce-to-hit case.
- Telemetry result:
  - `ricochet_attempts = 1`
  - `ricochet_surface_hits = 1`
  - `ricochet_enemy_hits = 1`
  - `ricochet_bonus_score = 22`
  - `last_projectile_event = RicochetHit`

### Hard Blocker

- Earlier BRP capture from this ticket session recorded the required crash case:
  - `hard_blocker_extrusion_crashes = 1`
  - `last_projectile_event = HardCrash`
  - `last_surface_archetype = HardCrashExtrusion`
- Fresh reruns could still query live hard-blocker entities, but teleport/input attempts were not stable enough to recreate collision-start timing consistently without adding extra debug hooks, which would have broadened scope.
- Reused artifacts:
  - `agents/deliverables/agent2/NB-A2-011_live.png`
  - `agents/deliverables/agent2/NB-A2-011_later.png`

## Validation Notes

- Ricochet validation was easiest through `NebulaValidationCommand`.
- Direct-hit validation needed BRP component mutation because surface-rich lanes often produced incidental bounces before the enemy contact.
- One BRP session crashed when mutating a despawned enemy entity; rerunning on a fresh process resolved the validation path without code changes.
