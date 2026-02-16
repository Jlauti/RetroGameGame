# Task Brief

## Metadata

- Ticket ID: NB-A4-004
- Agent: agent4
- Assigned By: principal_engineer
- Assigned Date: 2026-02-15
- Due Date: 2026-02-16

## Context

Current Nebula assets still read as placeholder concept exports in play. This task prioritizes practical in-game sprites for gameplay readability.

## Concrete Steps

1. Produce player sprite tuned for gameplay readability and motion cues.
2. Produce at least two enemy archetype sprites with clearly distinct silhouettes.
3. Produce tileable ground/wall sprites suitable for repeated in-game use.
4. Enforce transparent or chroma-key pipeline (no white backgrounds) and run background validator.
5. Update manifest with dimensions, pivot assumptions, and recommended runtime scale.

## Boundaries

- Follow ticket `Allowed Paths` only.
- No gameplay code changes.

## Acceptance

- Sprite pack delivered with naming consistency.
- Manifest addendum includes integration-critical details.
- `python assets/scripts/check_bg.py /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer --strict` passes.

## Report Format

Return report at:

`/home/jl/git/RetroGameGame/agents/reports/agent4/NB-A4-004_task_report.md`
