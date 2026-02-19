# NB-CX-009 - Nebula Sprite Orientation Wiring + Runtime Verification

## Metadata

- Ticket ID: NB-CX-009
- Owner Agent: codex_worker1
- Status: READY_TO_MERGE
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: agent2
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: ENGINE_INTEGRATION
- Start Date: 2026-02-17
- Scoped Test Command: cargo-safe check --bin retro-game-game

## Objective

Wire Aino's approved sprite metadata into runtime orientation behavior so misrotated assets can be corrected via config without gameplay-code rewrites.

## Allowed Paths

- src/eras/era_future/nebula_bouncer/components.rs
- src/eras/era_future/nebula_bouncer/mod.rs
- src/eras/era_future/nebula_bouncer/resources.rs
- src/eras/era_future/nebula_bouncer/systems.rs
- specs/future/nebula_bouncer/sprite_orientation.json
- specs/future/nebula_bouncer/README.md
- agents/reports/codex_worker1/

## Out of Scope

- No unrelated gameplay rebalance changes.
- No changes outside Nebula Future-era runtime wiring.

## Acceptance Commands

- cargo-safe check --bin retro-game-game
- cargo-safe fmt -- --check

## Definition of Done

- Runtime orientation uses config-backed offsets for player and orb.
- Overlay/debug output shows active orientation offsets.
- `sprite_orientation.json` exists and is documented.
- Ticket report submitted at `/home/jl/git/RetroGameGame/agents/reports/codex_worker1/NB-CX-009_task_report.md`.
