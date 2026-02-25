# NB-QA-020 - QA Signoff for Nebula Camera + Topography Visual Wave

## Metadata

- Ticket ID: NB-QA-020
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

Validate the camera-behind-ship and neon-hex topography wave for stability, readability, and behavior preservation.

## Allowed Paths

- agents/qa/
- agents/reports/qa/
- agents/backlog/NB-QA-020.md
- src/eras/era_future/nebula_bouncer/ (read-only)
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

- NB-A1-006 completed.
- NB-A2-008 completed.
- NB-A3-003 completed.
- NB-A4-011 completed.

## Definition of Done

- QA signoff artifact exists at `agents/qa/NB-QA-020_qa_signoff.md` with PASS/FAIL verdict.
- Command exit codes are recorded.
- Scope boundary check is recorded.
- HITL notes cover:
  - camera angle/framing ("behind ship" readability)
  - player and projectile readability against neon hex terrain
  - topography tier clarity during movement/combat
  - movement/collision/aiming regression status
