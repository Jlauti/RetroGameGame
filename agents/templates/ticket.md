# Ticket

## Metadata

- Ticket ID: NB-XXX-000
- Owner Agent: agent1
- Recommended Specialist: Aarne Tasapaino (Gameplay)
- Preferred Model: Codex
- Complexity: Medium
- Status: TODO
- Execution Lane: LOCAL
- Critical Path: YES
- Jules Eligible: NO
- Fallback Owner: agent1
- Retry Count: 0
- Session Health: HEALTHY
- Work Category: CORE_GAMEPLAY
- Routing Reason: optional when overriding the default complexity -> model mapping

## Objective

Define the concrete outcome for this ticket.

## Allowed Paths

- agents/deliverables/agent1/
- specs/nebula_bouncer.md

## Out of Scope

- No changes outside allowed paths.

## Acceptance Commands

- cargo check
- cargo test
- cargo fmt -- --check

## Dependencies

List required upstream tickets or assets.

## Definition of Done

- Deliverable file(s) produced.
- Acceptance commands pass.
- QA signoff is `PASS`.
