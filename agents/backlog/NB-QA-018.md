# NB-QA-018 - QA Signoff for Nebula Isometric 2.5D Migration

## Metadata

- Ticket ID: NB-QA-018
- Owner Agent: qa
- Status: TODO
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: qa
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: QA_VALIDATION
- Start Date: 2026-02-24

## Objective

Validate that Nebula Bouncer's isometric-style 2.5D migration is stable, readable, and behaviorally equivalent for core gameplay loops.

## Allowed Paths

- agents/qa/
- agents/reports/qa/
- agents/backlog/NB-QA-018.md
- src/eras/era_future/nebula_bouncer/ (read-only for verification)
- specs/nebula_bouncer.md (read-only)
- docs/architecture/DESIGN.md (read-only)

## Out of Scope

- No gameplay code changes.
- No art generation or model editing.

## Acceptance Commands

- cargo build --bin retro-game-game
- cargo test --lib nebula_bouncer
- cargo fmt -- --check

## Dependencies

- NB-A1-004 completed.
- NB-A2-006 completed.
- NB-A4-009 completed.

## Definition of Done

- QA artifact exists at `agents/qa/NB-QA-018_qa_signoff.md` with PASS/FAIL verdict.
- Command exit codes are recorded.
- Scope boundary check is recorded.
- HITL notes cover:
  - camera angle/readability in isometric view
  - aiming fidelity vs cursor position
  - movement/collision/projectile regression status
