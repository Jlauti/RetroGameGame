# HITL Runbook - 2026-02-17

## Objective

Validate current Future-era Nebula Bouncer behavior after merge-ready consolidation of:

1. `NB-CX-009` (sprite orientation config wiring)
2. `NB-CX-010` (release-readiness reconciler pipeline)

## Preflight

1. Ensure you are on the intended integration branch (`develop` or dedicated `hitl` branch).
2. Pull latest remote before running:
   - `git pull --ff-only`
3. Build once:
   - Strong machine path: `cargo build --bin retro-game-game`
   - Guardrailed local path: `cargo-safe build --bin retro-game-game`

## Launch

1. Run game:
   - Strong machine path: `cargo run --bin retro-game-game`
   - Guardrailed local path: `cargo-safe run --bin retro-game-game`
2. Optional verbose telemetry:
   - `RUST_LOG=info cargo run --bin retro-game-game`

## Test Checklist

1. Core movement and shooting
   - Input: `WASD` or arrow keys, left mouse click.
   - Pass if movement is responsive and orb spawns consistently.
2. Ricochet reliability
   - Fire at wall angles and confirm visible bounce direction change.
   - Pass if orb exits corner traps and continues with expected bounce behavior.
3. Orientation offset runtime wiring
   - Press `F8` and capture telemetry line with `player_offset_deg` and `orb_offset_deg`.
   - Edit `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/sprite_orientation.json`, relaunch, press `F8` again.
   - Pass if offsets in telemetry match config values and sprite facing visibly changes accordingly.
4. Screen shake sanity
   - Create repeated hits and then stop firing.
   - Pass if shake decays to zero; fail if camera keeps drifting/oscillating while idle.
5. Collision/hitbox sanity
   - Validate player, orb, and wall collisions feel consistent without ghost overlaps.
   - Pass if no persistent clipping or stuck-state occurs.

## Evidence To Capture

1. One screenshot during active combat.
2. One screenshot of a problematic case (if any).
3. Short text log:
   - `PASS/FAIL` per checklist item.
   - Repro steps for each failure.
   - Severity (`blocker`, `major`, `minor`).

## Exit Criteria

HITL pass is acceptable when all checklist items pass, or when only minor/non-blocking issues remain and are ticketed with clear repro.

