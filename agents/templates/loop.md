# Loop

## Metadata

- Loop ID: LOOP-YYYY-MM-DD-scope
- Name: Loop Name
- Owner: principal_engineer
- Status: PLANNED
- Value Hypothesis: One-line non-trivial value statement.
- Value Class: GAMEPLAY

## Scope In

- agents/backlog/NB-XXX-000.md
- src/eras/era_future/

## Scope Out

- Unrelated eras.
- Tooling unrelated to this loop.

## Tickets Included

- NB-XXX-000

## Worker Plan

- principal_engineer: orchestration + merges
- agent2: runtime implementation
- qa: signoff

## Acceptance Commands

- cargo-safe check --bin retro-game-game
- cargo-safe test --lib nebula_bouncer
- cargo-safe fmt -- --check

## Acceptance Evidence Required

- QA signoff files for included tickets
- Merge manifests for included tickets
- Runtime notes or screenshots proving value target

## Completion Gate

- Included tickets merged or explicitly deferred with reason
- Acceptance commands pass
- Required QA decisions are PASS
- Principal engineer marks loop COMPLETE
