# Task Brief

## Metadata

- Ticket ID: NB-CX-011
- Agent: codex_worker1
- Assigned By: principal_engineer
- Assigned Date: 2026-02-17
- Due Date: 2026-02-18

## Context

Nebula gameplay loop is currently functional, but visuals are still heavily debug-primitive driven. We need a large integration pass that wires Aino's sprite outputs into runtime without destabilizing gameplay and without blocking on perfect art completeness.

## Concrete Steps

1. Add a config-driven Nebula asset manifest at:
   - `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/asset_manifest.json`
2. Implement manifest/resource loading in Nebula runtime (`resources.rs`), including safe defaults when files are missing or malformed.
3. Replace debug primitive render usage with sprite-based rendering for:
   - player ship,
   - kinetic orb,
   - at least one enemy archetype.
4. Keep gameplay colliders and damage logic stable while swapping visuals.
5. Preserve fallback behavior:
   - if sprite handle/path is invalid, entity still renders with current primitive visual and log entry.
6. Ensure orientation integration remains correct with:
   - `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/sprite_orientation.json`
7. Update `/home/jl/git/RetroGameGame/specs/future/nebula_bouncer/README.md` with manifest format and tuning guidance.
8. Run acceptance commands and record exact exits.
9. Produce final report with:
   - changed files,
   - runtime behavior summary,
   - gate evidence and unresolved risks.

## Boundaries

- Allowed paths only.
- Do not generate or modify raw art assets in this ticket.
- Do not edit gate queue scripts.
- If blocked by missing assets, continue using fallback-first integration and document exactly what is missing.

## Acceptance

- `cargo-safe check --bin retro-game-game` passes.
- `cargo-safe test --lib nebula_bouncer` passes.
- `cargo-safe fmt -- --check` passes.
- Sprite manifest and runtime wiring are in place with graceful fallbacks.

## Report Format

Return report at:

`/home/jl/git/RetroGameGame/agents/reports/codex_worker1/NB-CX-011_task_report.md`

