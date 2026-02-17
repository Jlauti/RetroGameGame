Status: PASS

Summary
- Added config-driven sprite manifest wiring for Nebula runtime using `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/asset_manifest.json`.
- Implemented safe manifest loading + per-asset on-disk validation with fallback-first behavior in runtime resources.
- Swapped key debug visuals to sprite-backed rendering for:
  - player ship,
  - kinetic orb/projectile,
  - default enemy archetype,
  - wall tiles.
- Preserved gameplay/collision behavior by keeping all existing collider, rigid body, and damage logic unchanged while only replacing render-source selection.
- Extended telemetry (`F8`) to expose fallback state (`player_fallback`, `orb_fallback`, `enemy_fallback`, `wall_fallback`) plus orientation offsets.
- Updated Nebula spec README with manifest schema and fallback/tuning guidance.

Files Changed
- `/home/jl/git/RetroGameGame/src/eras/era_future/nebula_bouncer/resources.rs`
- `/home/jl/git/RetroGameGame/src/eras/era_future/nebula_bouncer/mod.rs`
- `/home/jl/git/RetroGameGame/src/eras/era_future/nebula_bouncer/systems.rs`
- `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/asset_manifest.json`
- `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/README.md`
- `/home/jl/git/RetroGameGame/agents/reports/codex_worker1/NB-CX-011_task_report.md`

Runtime Behavior
- Manifest path values are interpreted relative to `assets/` (or normalized from `assets/...` to asset-server paths).
- On startup, each manifest slot is resolved and validated against filesystem presence.
- Missing sprite files no longer block runtime: primitive visuals are used automatically and logged.
- Orientation remains config-backed through `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/sprite_orientation.json`.

Validation Results
- `cargo-safe check --bin retro-game-game` -> exit=0
- `cargo-safe test --lib nebula_bouncer` -> exit=0
- `cargo-safe fmt -- --check` -> exit=1 (rustfmt import-wrap diff only)
- `cargo-safe fmt` -> exit=0
- `cargo-safe fmt -- --check` -> exit=0
- Final reconfirmation after formatting:
  - `cargo-safe check --bin retro-game-game` -> exit=0
  - `cargo-safe test --lib nebula_bouncer` -> exit=0

Open Risks
- Asset presence checks are filesystem-relative to runtime cwd; non-standard launch cwd can force fallback mode even if art exists elsewhere.
- Manifest supports one default enemy sprite slot in this wave; per-enemy archetype mapping is deferred to a follow-up ticket.
- Sprite fallback state is telemetry/log-driven (F8 + startup logs), not yet surfaced in an always-on UI panel.

Recommended Next Step
1. Add per-archetype enemy sprite keys to the manifest and spawn-time archetype tagging for visual diversity without gameplay changes.
