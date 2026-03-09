# NB-FIX-005 Runtime Notes — Ground Read and Health Drop Hotfix

**Ticket**: NB-FIX-005
**Agent**: Pekka Kone (agent2)
**Date**: 2026-03-08

## Hotfix Scope

This hotfix stayed inside the ticket scope:

- removed the duplicated startup procgen/reset path that produced overlapping and camera-relative floor reads
- kept the procedural neon ground and boundary contracts intact
- made health-bearing breakable rewards readable once spawned, without introducing new pickup economy or new gameplay rules

## BRP/MCP-Assisted Validation Setup

- Booted with `RETRO_DEV_BOOT=nebula`
- Enabled BRP with `BEVY_BRP_ENABLE=1`
- Used BRP JSON-RPC methods:
  - `world.get_resources`
  - `world.query`
  - `world.insert_components`
  - `world.insert_resources`
  - `brp_extras/send_keys`
  - `brp_extras/screenshot`

Artifacts from the final runtime pass:

- `agents/deliverables/agent2/NB-FIX-005_ground_start.png`
- `agents/deliverables/agent2/NB-FIX-005_ground_move.png`
- `agents/deliverables/agent2/NB-FIX-005_health_drop.png`
- `agents/deliverables/agent2/NB-FIX-005_brp_validation.json`
- `agents/deliverables/agent2/NB-FIX-005_brp_run.out.log`
- `agents/deliverables/agent2/NB-FIX-005_brp_run.err.log`

## Case 1: Moving Gameplay With World-Anchored Ground

Captured before/after lateral movement using BRP `send_keys(["KeyD"], 450)`.

- Player translation: `[0.0, -200.0, 38.0]` -> `[135.9375, -200.0, 38.0]`
- Camera translation: `[0.0, -596.0, 270.0]` -> `[43.0672, -596.0, 270.0]`
- Tracked topography hex entity: `4294966697`
- Tracked hex translation: `[-430.5920, 205.6863, 6.6871]` -> `[-430.5920, 46.4027, 6.6871]`

Interpretation:

- the player and camera shifted laterally as expected
- the tracked floor hex remained a single coherent world entity with stable `x/z`, while the lane flow advanced in `y`
- screenshots show no startup overlap, camera-relative fragments, or duplicated floor slabs

## Case 2: Coherent Floor / Topography Read

`NebulaProcgenValidationState.recent` reported a monotonic startup sequence:

- `0 -> 400.0`
- `1 -> 1200.0`
- `2 -> 2000.0`
- `3 -> 2800.0`
- `4 -> 3600.0`
- `5 -> 4400.0`

This is the key runtime confirmation that the old "prefill, reset to zero, then spawn first chunk again" overlap is gone. The floor reads as one continuous authored stream instead of a restarted chunk stack.

## Case 3: Destroyed Health-Bearing Breakable Produces Visible Collectable Drop

BRP staging was used only for validation:

- switched the live loadout to `Plasma/Mass` via `F7` twice
- moved the single live health-bearing breakable into a clear lane line with `world.insert_components`
- triggered the normal fire path through `NebulaValidationCommand`

Observed runtime result:

- breakable entity: `4294965170`
- original translation: `[733.3120, 1771.2549, 5.0]`
- staged translation used for validation: `[180.0, 1738.3835, 5.0]`
- telemetry:
  - `breakables_destroyed = 1`
  - `health_breakables_destroyed = 1`
  - `last_breakable_family = PocketCluster`
  - `last_breakable_reward = HealthBearing`

Spawned reward:

- health drop entity: `4294962883`
- translation: `[180.0, 1701.8937, 72.2620]`
- component:
  - `heal_amount = 10`
  - `ttl_secs = 5.7806`
  - `collect_radius = 38.0`

`NB-FIX-005_health_drop.png` shows the spawned reward as a tall amber beacon that remains visible in-world after the breakable is destroyed.
