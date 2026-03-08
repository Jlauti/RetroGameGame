# Current Focus

- Updated: 2026-03-08
- Product Mode: `Nebula-first`
- Product Direction: Nebula Bouncer is a modern 3D third-person roguelite space shooter inspired by *Star Goose* (1988).
- Current Objective: finish enemy world-space movement and attack-facing behavior so Nebula enemies behave like incoming attackers instead of player-relative lane traffic.
- Current Phase: post-HITL enemy movement/orientation runtime follow-up

## Active Workstreams

| Owner | Focus | Status | Notes |
|------|-------|--------|-------|
| principal_engineer | Scope, sequencing, merge decisions, product focus | Active | Keep Nebula as the only live delivery lane |
| Aarne Tasapaino | Core combat loop, progression hooks, encounter rules | Active | Immediate handoff: define enemy combat roles, movement intent, and return-fire contract via `agents/backlog/NB-A1-009.md` |
| Pekka Kone | Third-person runtime support, camera/gameplay wiring, spawning/integration | Active | Immediate handoff: resolve enemy world-space movement and attack-facing via `agents/backlog/NB-A2-014.md` |
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

- Enemy locomotion must be world/ground-relative rather than player-throttle-relative.
- Enemies must stop reading as side-hovering same-speed traffic when the player is neutral.
- Enemies must visibly face the player while telegraphing and firing.
- Ground redesign remains deferred; this pass is enemy movement/orientation only.
- Current delegation order:
  - `Pekka Kone`: implement the enemy world-space movement and attack-facing ticket in `agents/backlog/NB-A2-014.md`

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
