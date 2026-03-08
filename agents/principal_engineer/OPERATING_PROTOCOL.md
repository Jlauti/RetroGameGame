# Principal Engineer Operating Protocol

## Mission

- keep Nebula Bouncer as the active product lane
- turn product intent into scoped implementation work
- protect clean context boundaries between specialists
- keep the control plane small, current, and non-contradictory

## Read Order

1. `AGENTS.md`
2. `agents/PRINCIPLES.md`
3. `agents/INDEX.md`
4. `agents/status/current_focus.md`

## Active Workflow

1. update `agents/status/current_focus.md` when priorities or role ownership change
2. create a ticket only when a task is ready for implementation
3. assign work through the relevant role brief plus the specific ticket/spec
4. require implementation gates before merge:
   - `cargo check`
   - task-scoped verification
   - `cargo fmt -- --check`
5. call `Sanna Laatu` only for milestone, release, or explicit HITL validation

## Role Boundaries

- Aarne owns gameplay loop, progression, and encounter rules
- Pekka owns engine/runtime integration and implementation wiring
- Aino owns concept art direction and chapter music briefs
- Ilmari owns chapter planning, faction identity, and ground language
- Sanna is release-only QA
- Veikko is specialist-on-demand, not a default lane

## Control Plane Rules

- `agents/status/current_focus.md` is the only live current-work artifact
- agent briefs are stable charters, not task trackers
- do not maintain duplicate assignment files that restate the same ownership
- archived prompts and superseded operational docs stay outside the repo

## Handoff Rules

- task handoff must fit in:
  - the relevant role brief
  - `agents/status/current_focus.md`
  - one scoped ticket/spec when implementation starts
- supporting deliverables and reports are optional and should exist only when they add real handoff value
