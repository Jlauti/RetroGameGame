# Task Brief

## Metadata

- Ticket ID: NB-A5-006
- Agent: agent5
- Assigned By: principal_engineer
- Assigned Date: 2026-02-25
- Due Date: 2026-02-27

## Context

Settings wave requires live music-volume control tied to persisted user settings. Current hub music flow exists, but runtime binding to user-configurable settings is missing.

## Concrete Steps

1. Integrate music-volume setting from settings runtime model into audio sink control.
2. Ensure volume updates apply live while panel is open.
3. Ensure volume persists and restores across app restart per contract.
4. Preserve non-crashing behavior when AudioSink is unavailable.
5. Document conversion policy (UI range to linear volume), edge cases, and test notes.

## Boundaries

- Follow ticket `Allowed Paths` only.
- No gameplay/mechanics changes.
- Avoid refactors outside audio/settings integration scope.

## Acceptance

- Music volume behavior is deterministic and persistent.
- Missing-device path remains warning-only.
- Command gates pass.

## Report Format

Return report at:

`c:\Users\jlaut\git\RetroGameGame/agents/reports/agent5/NB-A5-006_task_report.md`
