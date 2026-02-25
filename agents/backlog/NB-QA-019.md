# NB-QA-019 - QA Signoff for Settings Panel Wave

## Metadata

- Ticket ID: NB-QA-019
- Owner Agent: qa
- Status: TODO
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: qa
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: QA_VALIDATION
- Start Date: 2026-02-25

## Objective

Validate the new Settings panel wave with emphasis on entry points, display changes, volume behavior, and quit flow stability.

## Allowed Paths

- agents/qa/
- agents/reports/qa/
- agents/backlog/NB-QA-019.md
- src/ui/ (read-only)
- src/core/ (read-only)
- src/main.rs (read-only)
- specs/nebula_bouncer.md (read-only)
- docs/architecture/DESIGN.md (read-only)

## Out of Scope

- No gameplay code changes.
- No asset generation/editing.

## Acceptance Commands

- cargo build --bin retro-game-game
- cargo test --lib nebula_bouncer
- cargo fmt -- --check

## Dependencies

- NB-A1-005 completed.
- NB-A2-007 completed.
- NB-A4-010 completed.
- NB-A5-006 completed.

## Definition of Done

- QA signoff artifact exists at `agents/qa/NB-QA-019_qa_signoff.md` with PASS/FAIL verdict.
- Command exit codes are recorded.
- Scope boundary check is recorded.
- HITL notes cover:
  - settings entry from main carousel/hub menu flow
  - `Esc` opens settings while in active gameplay
  - resolution and display mode transitions (including windowed-fullscreen and fullscreen)
  - music volume live change + persistence check
  - quit action behavior from settings panel
