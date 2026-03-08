# NB-A2-011 Task Report

## Scope Delivered

- Added explicit surface-role metadata for Nebula terrain, soft boundaries, ricochet extrusions, and hard-crash extrusions.
- Added terrain-follow visual response on the player visual root without changing the gameplay plane, cursor aiming path, or camera framing logic.
- Reworked player/runtime collision handling so traversal remains non-lethal, soft boundaries pressure/steer the ship, and only explicit hard blockers can end the run.
- Reworked projectile/runtime collision handling so only approved surfaces ricochet, direct fire remains the baseline path, ricochet rewards stay bounded, and bounce loops/speed escalation are clamped.
- Added `NebulaRuntimeTelemetry` and `NebulaValidationCommand` so BRP can validate terrain follow, direct fire, ricochet behavior, and blocker outcomes against live runtime state.

## Validation

- `cargo check`: passed
- `cargo build --bin retro-game-game`: passed
- `cargo test --lib nebula_bouncer`: passed
- `cargo fmt -- --check`: passed

- BRP boot used `RETRO_DEV_BOOT=nebula`, `BEVY_BRP_ENABLE=1`, `BEVY_BRP_PORT=15702`.
- Terrain-follow/control proof:
  - BRP key injection moved the ship from `[17.2254, -1208.7845, 38.0]` to `[126.0199, -1245.0757, 38.0]`.
  - Telemetry recorded `max_skim_height = 0.2508809`.
  - Screenshot saved to `agents/deliverables/agent2/NB-A2-011_terrain_follow.png`.
- Direct-fire proof:
  - BRP reactivated a pooled orb on a live enemy with `ricochet_count = 0` to isolate the direct-hit runtime path.
  - Telemetry recorded `direct_enemy_hits = 1`, `ricochet_surface_hits = 0`, `last_projectile_event = DirectHit`.
- Ricochet proof:
  - BRP validation-command shot captured `ricochet_attempts = 1`, `ricochet_surface_hits = 1`, `ricochet_enemy_hits = 1`, `last_projectile_event = RicochetHit`.
- Hard-blocker proof:
  - Earlier BRP capture from this same ticket run recorded `hard_blocker_extrusion_crashes = 1`, `last_projectile_event = HardCrash`, `last_surface_archetype = HardCrashExtrusion`.
  - Later reruns could query hard blockers reliably, but teleport/input reproduction was inconsistent because BRP teleports do not reliably recreate collision-start timing.

## Deliverables

- Runtime notes: `agents/deliverables/agent2/NB-A2-011_runtime_notes.md`
- BRP screenshots/logs:
  - `agents/deliverables/agent2/NB-A2-011_terrain_follow.png`
  - `agents/deliverables/agent2/NB-A2-011_initial.png`
  - `agents/deliverables/agent2/NB-A2-011_after_retry.png`
  - `agents/deliverables/agent2/NB-A2-011_live.png`
  - `agents/deliverables/agent2/NB-A2-011_later.png`
  - `agents/deliverables/agent2/NB-A2-011_brp_run.out.log`
  - `agents/deliverables/agent2/NB-A2-011_brp_run.err.log`

## Notes

- One BRP session panicked when a stale despawned enemy entity was mutated. The code under test was unchanged; validation was rerun on a fresh process and the final code gates stayed green.
- I did not add extra runtime hooks beyond the ticket scope. The validation path used the telemetry/resource support added by this ticket plus BRP component/resource mutation where cursor-driven aiming proved unreliable.
