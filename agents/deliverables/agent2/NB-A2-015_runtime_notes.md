# NB-A2-015 Runtime Notes

Date: 2026-03-08
Ticket: `NB-A2-015`
Mode: `Nebula-first`

## BRP Launch Notes

- Successful Windows launch used `cmd.exe` to start the built binary with:
  - `RETRO_DEV_BOOT=nebula`
  - `BEVY_BRP_ENABLE=1`
  - `BEVY_BRP_PORT=15702`
- BRP validation was run against `http://127.0.0.1:15702`.
- Runtime inspection used BRP JSON-RPC:
  - `world.get_resources` for `NebulaProcgenValidationState` and `NebulaRuntimeTelemetry`
  - `world.query` for live enemy role/transform sampling

## Ticket-Scoped Runtime Validation

### 1. Combat-pocket case

Primary capture came from `NebulaProcgenValidationState.recent`:

- `sequence_index: 3`
- `chunk_center_y: 2700.0`
- `cadence: CombatPocket`
- `enemy_count: 2`
- `breakable_hazard_clusters: 2`
- `health_bearing_breakables: 1`
- `shoulder_breakables: 1`
- `cage_adjacent_breakables: 1`
- `core_lane_breakables: 0`
- `shoulder_ricochet_banks: 28`

Result: the runtime emitted a combat pocket with the required mixed support-target layout: one shoulder breakable plus one pocket-side/cage-adjacent breakable, capped to enemy count and keeping the core lane clear.

Live enemy BRP query from the same validation session returned active `Flanker`, `Sniper`, and `Blocker` roles. The chunk snapshot encodes enemy count, not per-chunk enemy roles, so the `mixed enemies` read is an inference from the concurrent live enemy-role sample plus the combat-pocket chunk count.

### 2. Relief-lane case

Validation capture:

- `sequence_index: 5`
- `chunk_center_y: 3199.5552`
- `cadence: ReliefLane`
- `enemy_count: 0`
- `breakable_hazard_clusters: 0`
- `destructible_surfaces: 0`
- `core_lane_surfaces: 6`
- `shoulder_surfaces: 0`
- `cage_adjacent_surfaces: 5`
- `ricochet_surfaces: 0`

Result: the relief lane read as a decompressed traversal beat. Clutter dropped to zero destructibles, the core lane stayed explicit, and the lane avoided combat-pocket density.

### 3. Lane-pressure case

Validation capture:

- `sequence_index: 4`
- `chunk_center_y: 3500.0`
- `cadence: LanePressure`
- `breakable_hazard_clusters: 2`
- `shoulder_breakables: 1`
- `cage_adjacent_breakables: 1`
- `core_lane_breakables: 0`
- `favored_side_sign: -1`
- `shoulder_surfaces: 11`
- `cage_adjacent_surfaces: 9`

Result: lane pressure biased geometry and support targets to one side instead of distributing clutter uniformly. Meaningful combat fixtures sat in `Shoulder` and `CageAdjacent` zones, while the cage itself remained containment rather than the scoring surface.

### 4. Ricochet-bank case

Validation capture used the same combat-pocket sample as case 1 because it showed the strongest optional bank-shot read without collapsing direct fire:

- `sequence_index: 3`
- `cadence: CombatPocket`
- `shoulder_ricochet_banks: 28`
- `ricochet_surfaces: 28`
- `core_lane_breakables: 0`
- `shoulder_breakables: 1`
- `cage_adjacent_breakables: 1`

Result: ricochet surfaces were concentrated on shoulder banks, while direct-fire readability stayed intact because the core lane had no breakable clutter and the support targets stayed off the lane center.

## Runtime Telemetry Note

- Validation focused on procgen/runtime layout semantics, not a manual playthrough.
- At capture time, `NebulaRuntimeTelemetry` showed zero live breakable hits/destroys, so this note validates placement/cadence semantics rather than a fired-orb destruction pass.
