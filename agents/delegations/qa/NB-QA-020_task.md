# Task Brief

## Metadata

- Ticket ID: NB-QA-020
- Agent: qa
- Assigned By: principal_engineer
- Assigned Date: 2026-02-25
- Due Date: 2026-02-28

## Context

Validate the Nebula graphics wave introducing a lower behind-ship camera and neon-hex topography terrain representation.

## Concrete Steps

1. Run ticket acceptance commands and record exit codes.
2. Run scope boundary verification.
3. Execute HITL checks:
   - behind-ship camera readability
   - player/projectile readability on neon hex terrain
   - topography tier clarity under combat motion
   - aiming/movement/collision regression check
4. Produce QA signoff artifact with PASS/FAIL verdict and rationale.

## Boundaries

- Follow ticket `Allowed Paths` only.
- No gameplay code edits.

## Acceptance

- QA signoff + report artifacts are complete.
- Command outputs and HITL evidence are documented.

## Report Format

Return report at:

`c:\Users\jlaut\git\RetroGameGame/agents/reports/qa/NB-QA-020_task_report.md`
