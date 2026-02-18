# Loop - Nebula Gameplay + Assetization

## Metadata

- Loop ID: LOOP-2026-02-18-nebula-gameplay-assetization
- Name: Nebula Gameplay Assignment And Assetization Loop
- Owner: principal_engineer
- Status: ACTIVE
- Value Hypothesis: Deterministic enemy and terrain assignment plus stronger runtime asset usage increases gameplay engagement and visual clarity.
- Value Class: GAMEPLAY

## Scope In

- agents/backlog/NB-CX-009.md
- agents/backlog/NB-CX-011.md
- agents/backlog/NB-A2-003.md
- src/eras/era_future/nebula_bouncer/
- specs/future/nebula_bouncer/

## Scope Out

- Non-Nebula eras.
- Broad rendering engine refactors.

## Tickets Included

- NB-CX-009
- NB-CX-011
- NB-A2-003

## Worker Plan

- principal_engineer: loop definition, merge sequencing, completion decision
- codex_worker1: Nebula runtime integration and orientation/asset wiring
- agent2: runtime asset integration hardening and follow-up fixes
- qa: ticket-level and loop-level signoff

## Acceptance Commands

- cargo-safe check --bin retro-game-game
- cargo-safe test --lib nebula_bouncer
- cargo-safe fmt -- --check

## Acceptance Evidence Required

- Passing gate history entries under `agents/status/gates/queue/history/`
- Updated release board showing merge-ready/merged progression
- QA PASS artifacts for included tickets
- Principal context updated with loop status and next actions

## Completion Gate

- Included tickets are merged to `develop` or deferred with explicit rationale
- Required QA signoffs are PASS
- Acceptance commands are green for final integration state
- Principal engineer marks this loop COMPLETE with final evidence note
