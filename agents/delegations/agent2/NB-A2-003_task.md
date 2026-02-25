# Task Brief

## Metadata

- Ticket ID: NB-A2-003
- Agent: agent2
- Assigned By: principal_engineer
- Assigned Date: 2026-02-15
- Due Date: 2026-02-16

## Context

After the new core sprite pack lands, runtime entities should use those assets directly for realistic playtesting.

## Concrete Steps

1. Replace placeholder color blocks with asset-backed sprites for player/enemies/walls/projectiles.
2. Apply consistent scaling and layering based on art manifest guidance.
3. Keep physics/collision behavior intact while swapping visuals.
4. Document entity-to-asset mapping and runtime caveats.

## Boundaries

- Follow ticket `Allowed Paths` only.
- No broad refactors outside `era_future`.

## Acceptance

- Merge gates pass.
- Integration mapping documented.

## Report Format

Return report at:

`c:\Users\jlaut\git\RetroGameGame/agents/reports/agent2/NB-A2-003_task_report.md`
