# Current Focus

- Updated: 2026-03-09
- Product Mode: `Nebula-first`
- Product Direction: Nebula Bouncer is a modern 3D third-person roguelite space shooter inspired by *Star Goose* (1988).
- Current Objective: close out the accepted ground/runtime hotfix work and prepare the next scoped Nebula improvement round.
- Current Phase: post-hotfix cleanup and next-round planning

## Active Workstreams

| Owner | Focus | Status | Notes |
|------|-------|--------|-------|
| principal_engineer | Scope, sequencing, merge decisions, product focus | Active | Keep Nebula as the only live delivery lane |
| Aarne Tasapaino | Core combat loop, progression hooks, encounter rules | Active | Ground-and-boundary contract completed via `agents/backlog/NB-A1-010.md` |
| Pekka Kone | Third-person runtime support, camera/gameplay wiring, spawning/integration | Active | Ground/runtime hotfix accepted; awaiting next scoped implementation ticket |
| Aino Kuvitus | Chapter concept art sheets and music briefs | Active | Procedural neon terrain visual language completed via `agents/backlog/NB-A4-013.md` |
| Ilmari Maasto | Chapter plans, faction identity, enemy families, ground language | Active | Define what appears in each chapter and why |
| Sanna Laatu | Release/HITL validation only | Standby | Activate only for milestone or release review |
| Veikko Fiilis | VFX/audio implementation specialist | Dormant | Activate only when dedicated polish work is intentionally scheduled |

## Chapter Planning Baseline

- Chapter planning starts from three enemy factions:
  - scrapper faction building ships from junk
  - biomechanical alien faction
  - sleek futuristic faction
- Each chapter brief should define:
  - faction owner
  - enemy families
  - ground identity
  - encounter intent
  - art concept needs
  - music brief

## Immediate Priority

- `NB-FIX-006` is accepted and ready to land.
- Nebula remains the only live delivery lane.
- The repo is clear to define the next scoped improvement ticket.
- No new health-bar/HUD requirement has been adopted as a product decision from this pass.
- Current delegation order:
  - none; next ticket to be defined by principal sequencing

## Ticketing Rule

- Create a ticket only when a scoped implementation task is ready to execute.
- Do not maintain placeholder task files just to mirror role ownership.

## Acceptance Defaults

- Required engineering gates for implementation tickets:
  - `cargo check`
  - ticket-scoped verification command
  - `cargo fmt -- --check`
- QA artifacts are required only for release, milestone, or explicit HITL checkpoints.

## Release Checkpoint

- Independent QA is not part of normal ticket flow.
- When a release or milestone checkpoint is called, Sanna validates the integrated Nebula build and records a single release/HITL artifact.
