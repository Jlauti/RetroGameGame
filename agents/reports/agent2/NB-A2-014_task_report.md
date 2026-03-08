# NB-A2-014 Task Report

Date: 2026-03-08
Agent: `agent2`
Ticket: `NB-A2-014`
Status: Complete

## Scope Implemented

Implemented only the Nebula enemy runtime changes required by the ticket:

- captured enemy attack depth in world space when engagement begins so active enemies stop re-targeting directly off the player’s latest forward/back depth
- preserved role-specific lane separation by locking each enemy’s engagement lane at capture time
- added live enemy attack-facing updates during `Telegraphing` and `Firing`
- kept telegraph visual alignment in sync with the updated enemy facing
- added narrow regression tests for locked anchor behavior and enemy-facing math

No new combat roles, ground redesign, non-Nebula refactors, or speculative combat behavior were added.

## Files Changed

- `src/eras/era_future/nebula_bouncer/components.rs`
- `src/eras/era_future/nebula_bouncer/mod.rs`
- `src/eras/era_future/nebula_bouncer/systems.rs`
- `agents/deliverables/agent2/NB-A2-014_runtime_notes.md`
- `agents/reports/agent2/NB-A2-014_task_report.md`

## Validation

Required cargo gates run:

- `cargo check`
  - passed
- `cargo build --bin retro-game-game`
  - passed
- `cargo test --lib nebula_bouncer`
  - passed, `34` tests
- `cargo fmt -- --check`
  - passed

Ticket-scoped runtime validation run with BRP:

- Nebula booted with `RETRO_DEV_BOOT=nebula`
- BRP enabled with `BEVY_BRP_ENABLE=1`
- Live validation used BRP JSON-RPC `world.query` and `world.insert_components` against `http://127.0.0.1:15702`
- Runtime evidence is stored in `agents/deliverables/agent2/NB-A2-014_runtime_notes.md`

## Outcome Summary

- Enemy movement no longer re-locks to the player’s latest W/S depth every frame; attack anchors remain world-captured after engagement begins.
- Shoulder attackers continue advancing into the hostile-fire loop instead of reading as side traffic matched to the player.
- Telegraphing and firing enemies now face the player live, and the telegraph visual follows the same attack bearing.
- Existing hostile-fire loop, attack states, and role separation remained intact after the change.

## Notes

- During BRP validation, the first runtime-facing issue found was a Bevy query borrow panic in the new attack-facing system. That was fixed before final validation.
- Windows BRP launch in this workspace required `cmd.exe` environment setup for the `.exe` process; Git Bash inline env assignment was not reliable here.
- Principal engineer should update `AGENTS.md` and `agents/team/pekka_kone/brief.md` with BRP/MCP knowhow captured in the runtime notes so future agents can use the path directly.
