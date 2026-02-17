# Task Brief

## Metadata

- Ticket ID: NB-A4-006
- Agent: agent4
- Assigned By: principal_engineer
- Assigned Date: 2026-02-17
- Due Date: 2026-02-18

## Context

Nebula now has runtime hooks ready for textured gameplay entities. This task focuses on shipping cleaner production sprites and metadata so integration tuning can be deterministic.

## Concrete Steps

1. Generate/refine gameplay sprite candidates with strict top-down orientation (facing up/north).
2. Produce at least 3 candidates per critical ship class before selecting final canon.
3. Promote approved assets into canonical filenames under `assets/sprites/future/nebula_bouncer/`.
4. Reject assets with stars/UI/text/diagonal bias/internal green artifacts.
5. Update review metadata and inspector artifacts in `agents/art/reviews/NB-A4-006_sprite_metadata.json` and `agents/art/reviews/NB-A4-006_sprite_inspector.html`.
6. Produce `agents/deliverables/agent4/sprite_metadata_v2.json` with per-sprite: filename, class, intended in-game size range, pivot, orientation note, `orientation_offset_deg`.
7. Run strict background validation before report.

## Boundaries

- Follow ticket `Allowed Paths` only.
- No gameplay code changes.

## Acceptance

- Canonical sprite set updated and reviewable.
- Review board and metadata files delivered for integration.
- `python assets/scripts/check_bg.py /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer --strict` passes.

## Report Format

Return report at:

`/home/jl/git/RetroGameGame/agents/reports/agent4/NB-A4-006_task_report.md`
