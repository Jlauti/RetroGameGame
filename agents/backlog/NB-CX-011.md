# NB-CX-011 - Nebula Sprite Runtime Integration Wave 1

## Metadata

- Ticket ID: NB-CX-011
- Owner Agent: codex_worker1
- Status: READY_TO_MERGE
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: codex_worker2
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: ENGINE_INTEGRATION
- Start Date: 2026-02-17
- Scoped Test Command: cargo-safe test --lib nebula_bouncer

## Objective

Integrate approved Future-era sprites into Nebula Bouncer runtime so player/enemy/projectile visuals are no longer debug-only primitives, while preserving gameplay collision behavior and guaranteeing fallback behavior if an asset is missing.

## Allowed Paths

- src/eras/era_future/nebula_bouncer/components.rs
- src/eras/era_future/nebula_bouncer/mod.rs
- src/eras/era_future/nebula_bouncer/resources.rs
- src/eras/era_future/nebula_bouncer/systems.rs
- src/eras/era_future/nebula_bouncer/procgen.rs
- specs/future/nebula_bouncer/asset_manifest.json
- specs/future/nebula_bouncer/sprite_orientation.json
- specs/future/nebula_bouncer/README.md
- agents/reports/codex_worker1/
- agents/delegations/codex_worker1/
- agents/prompts/
- agents/backlog/NB-CX-011.md

## Out of Scope

- No new art generation in this ticket.
- No edits to cargo gate scripts or queue pipeline.
- No changes outside Nebula Future-era runtime and ticket artifacts listed above.

## Acceptance Commands

- cargo-safe check --bin retro-game-game
- cargo-safe test --lib nebula_bouncer
- cargo-safe fmt -- --check

## Dependencies

- Existing Aino asset outputs under `assets/sprites/future/`.
- Orientation offsets from `specs/future/nebula_bouncer/sprite_orientation.json`.

## Definition of Done

- Player, orb, and at least one enemy archetype render through sprite assets via config-driven paths.
- Runtime keeps existing collision/hitbox gameplay behavior (visual swap only, unless explicitly justified).
- Missing assets fail gracefully to debug primitive fallback with clear log/telemetry evidence.
- Asset manifest exists and is documented for future art iteration.
- Task report written at `/home/jl/git/RetroGameGame/agents/reports/codex_worker1/NB-CX-011_task_report.md` with command exits and file list.
