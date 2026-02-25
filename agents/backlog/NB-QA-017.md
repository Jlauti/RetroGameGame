# NB-QA-017 - QA Signoff for Nebula 3D Model Integration Wave 1

## Metadata

- Ticket ID: NB-QA-017
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

Validate that Nebula Bouncer player/enemy `.glb` integrations are stable, correctly oriented in top-down play, and gate-clean.

## Allowed Paths

- agents/qa/
- agents/reports/qa/
- agents/backlog/NB-QA-017.md
- src/eras/era_future/nebula_bouncer/ (read-only for verification)
- specs/future/nebula_bouncer/ (read-only for verification)

## Out of Scope

- No gameplay code changes.
- No asset generation or asset editing.

## Acceptance Commands

- cargo build --bin retro-game-game
- cargo test --lib nebula_bouncer
- cargo fmt -- --check

## Dependencies

- NB-A4-008 completed.
- NB-A2-004 completed.
- NB-A2-005 completed (if included in this QA pass).

## Definition of Done

- QA artifact exists at `agents/qa/NB-QA-017_qa_signoff.md` with PASS/FAIL verdict.
- Command exit codes are recorded.
- Scope boundary check is recorded.
- HITL notes include:
  - model visibility and lighting
  - top-down orientation sanity
  - movement/collision/shooting regression check
