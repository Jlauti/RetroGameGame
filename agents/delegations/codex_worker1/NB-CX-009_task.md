# Task Brief

## Metadata

- Ticket ID: NB-CX-009
- Agent: codex_worker1
- Assigned By: principal_engineer
- Assigned Date: 2026-02-17
- Due Date: 2026-02-18

## Context

Art production is accelerating and sprite forward axes may vary between batches. Runtime must not require code edits whenever a sprite arrives with a different facing direction.

## Concrete Steps

1. Add or finalize config-backed orientation offsets for Nebula player/orb facing.
2. Ensure defaults match top-down north-facing art while allowing easy override.
3. Surface active offsets in debug overlay so HITL can verify calibration quickly.
4. Keep scope strictly inside ticket allowed paths.
5. Run acceptance commands and report exact exit codes.

## Boundaries

- Allowed paths only.
- No physics/gameplay behavior rewrites outside orientation wiring.
- Use `cargo` for all Cargo commands.

## Acceptance

- `cargo check --bin retro-game-game` passes.
- `cargo fmt -- --check` passes.
- Report includes any remaining rotation risks and follow-up notes for Aino metadata handoff.

## Report Format

Return report at:

`c:\Users\jlaut\git\RetroGameGame/agents/reports/codex_worker1/NB-CX-009_task_report.md`
