# Current Focus

- Updated: 2026-03-08
- Product Mode: `Nebula-first`
- Product Direction: Nebula Bouncer is a modern 3D third-person roguelite space shooter inspired by *Star Goose* (1988).
- Current Objective: remove the remaining enemy presentation/runtime regressions so Nebula enemies read as clean world-space threats.
- Current Phase: post-HITL enemy presentation and world-motion hotfix delegation

## Active Workstreams

| Owner | Focus | Status | Notes |
|------|-------|--------|-------|
| principal_engineer | Scope, sequencing, merge decisions, product focus | Active | Keep Nebula as the only live delivery lane |
| Aarne Tasapaino | Core combat loop, progression hooks, encounter rules | Active | Immediate handoff: define enemy combat roles, movement intent, and return-fire contract via `agents/backlog/NB-A1-009.md` |
| Pekka Kone | Third-person runtime support, camera/gameplay wiring, spawning/integration | Active | Immediate hotfix: resolve enemy presentation and world-motion regression via `agents/backlog/NB-FIX-004.md` |
| Aino Kuvitus | Chapter concept art sheets and music briefs | Active | Produce 2D planning artifacts that support later 3D modeling |
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

- Remove the leftover red enemy box artifact from gameplay presentation.
- Enemy pre-engagement motion must not read as backward drift or same-lane traffic.
- Enemy locomotion must be stable relative to the world/ground and not directly follow player `W`/`S` throttle changes.
- The corrected presentation/motion pass must preserve the already-fixed hostile-fire damage path, projectile cap, combat-token behavior, and improved facing.
- Current delegation order:
  - `Pekka Kone`: implement the enemy presentation and world-motion hotfix in `agents/backlog/NB-FIX-004.md`

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
