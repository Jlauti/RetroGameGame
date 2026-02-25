# Task Brief

## Metadata

- Ticket ID: NB-A2-010
- Agent: agent2
- Assigned By: principal_engineer
- Assigned Date: 2026-02-25
- Due Date: 2026-02-27

## Context

This is a standalone single-agent waterfall implementation, not a multi-agent wave.

Current Nebula visuals are still failing intent:
- perceived diagonal scroll drift
- camera framing feels too far from ship/ground
- terrain reads as blocky rectangular bands instead of true hex topography
- relief language is weak and style cohesion is below target

Your job is to execute one sequential implementation pass that resolves these issues while preserving runtime stability and deterministic procgen behavior.

## Execution Order (Waterfall, Do Not Parallelize)

1. **Baseline + Safety**
- Capture baseline screenshots at `1920x1080`.
- Record existing camera/scroll/topography constants.
- Create a local checkpoint commit before substantial edits.

2. **Forward Scroll Lock**
- Ensure scrolling reads straight forward only in runtime view.
- Validate chunk movement and spawn progression alignment.
- Remove any framing/vector choices that visually reintroduce diagonal travel.

3. **Camera Reframe**
- Move camera closer to ship and ground plane.
- Keep lower-third ship framing and enough forward lookahead for gameplay.
- Preserve cursor-to-world aiming integrity.

4. **True Hex Ground Pass**
- Ensure topography footprint reads as actual hex tiles, not broad rectangles.
- Keep tile silhouette legible in motion and at gameplay zoom.

5. **Gentle Relief Pass**
- Implement dips/mounds with smooth transitions and restrained amplitude.
- No steep occluding geometry; readability first.

6. **Neon Palette/Readability Pass**
- Deliver clear multi-color neon tier differentiation.
- Keep terrain subordinate to player/enemy/projectile readability.

7. **Validation + Docs**
- Run full acceptance commands.
- Capture after screenshots at `1920x1080`.
- Write runtime notes and task report.
- Create final commit.

## Exact File Targets

- `src/eras/era_future/nebula_bouncer/systems.rs`
  - `setup_nebula_bouncer`
  - `update_level_scrolling`
  - `spawn_next_chunk`
- `src/eras/era_future/nebula_bouncer/topography.rs`
  - `TIER_COLORS`
  - `spawn_chunk_topography`
  - height smoothing/elevation mapping helpers
- `src/eras/era_future/nebula_bouncer/resources.rs`
  - shared mesh/material handles only if required for true hex rendering
- `src/eras/era_future/nebula_bouncer/procgen.rs`
  - only if needed for topography data-shape support; determinism must remain unchanged
- `specs/nebula_bouncer.md`
- `docs/architecture/DESIGN.md`

## Boundaries

- Follow ticket `Allowed Paths` only.
- No non-Nebula refactors.
- No new gameplay systems.
- No asset authoring/replacement for ship models.

## Acceptance

- `cargo build --bin retro-game-game`
- `cargo test --lib nebula_bouncer`
- `cargo fmt -- --check`
- HITL at 16:9 confirms:
  - forward-only scroll read
  - closer chase framing
  - clear hex topography silhouettes
  - gentle relief (dips/mounds)
  - readable neon tier differentiation

## Required Deliverables

- `agents/deliverables/agent2/NB-A2-010_visual_overhaul_notes.md`
  - include constants changed, before/after notes, and screenshots reference list
- `agents/reports/agent2/NB-A2-010_task_report.md`
  - include gates run, outcomes, and residual risks

## Report Format

Return report at:

`c:\Users\jlaut\git\RetroGameGame/agents/reports/agent2/NB-A2-010_task_report.md`
