# Merge Order Wave 3 - 2026-02-17

## Scope

Consolidated merge-prep wave after QA completion, focused on codex tickets now marked merge-ready.

## Candidate Set

1. `NB-CX-007`
2. `NB-CX-008`
3. `NB-CX-009`
4. `NB-CX-010`

## Proposed Order

1. `NB-CX-010` (release-readiness tooling; non-gameplay)
2. `NB-CX-008` (procgen soak/weights)
3. `NB-CX-007` (synergy runtime wiring)
4. `NB-CX-009` (orientation offset runtime wiring)

## Rationale

1. Land tooling first to preserve reconciliation visibility while merging.
2. Apply procgen before broader gameplay runtime surfaces.
3. Apply synergy runtime before orientation polish to reduce conflict scope in `systems.rs`.

## Preconditions

1. TICKET-level evidence present in manifest.
2. QA signoff `PASS` present in `/home/jl/git/RetroGameGame/agents/qa/`.
3. One MERGE-mode gate run on the final combined staging branch before main promotion.

## Required Finalization

1. Update each merge manifest with final merge gate artifact.
2. Set `Merge Decision: MERGED` only after successful merge to target branch.

