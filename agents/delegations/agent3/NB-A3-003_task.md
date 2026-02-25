# Task Brief

## Metadata

- Ticket ID: NB-A3-003
- Agent: agent3
- Assigned By: principal_engineer
- Assigned Date: 2026-02-25
- Due Date: 2026-02-27

## Context

Nebula needs deterministic topography metadata per chunk so runtime can render neon hex terrain tiers consistently.

## Concrete Steps

1. Extend Nebula procgen schema/resources with topography tier data output.
2. Ensure topography generation is deterministic by seed/chunk identity.
3. Add at least one focused validation/test for determinism and tier bounds.
4. Document runtime-facing handoff for agent2:
   - data fields
   - expected ranges
   - example mapping notes
5. Produce deliverable and task report.

## Boundaries

- Follow ticket `Allowed Paths` only.
- No camera code work.
- No broad gameplay tuning.

## Acceptance

- Command gates pass.
- Procgen output and test evidence are present.
- Handoff notes are clear enough for runtime integration.

## Report Format

Return report at:

`c:\Users\jlaut\git\RetroGameGame/agents/reports/agent3/NB-A3-003_task_report.md`
