# Agent Execution Guardrails

This repository uses a small named agent team. The active delivery model is `Nebula-first`: Nebula Bouncer is the only live product lane unless the principal engineer explicitly says otherwise.

## Active Control Plane

Read only the files that are relevant to your role:

1. `AGENTS.md`
2. `agents/PRINCIPLES.md`
3. your own `agents/team/<codename>/brief.md`
4. `agents/status/current_focus.md`
5. only the specific ticket/spec files linked from your brief or assigned task

Do not read archived prompts, unrelated agent briefs, or broad historical docs by default.

## Build Rules

- **Cargo**: Use plain `cargo` for all build/test/check operations
- **Python**: Use `py` (not `python3`)
- **Platform**: Windows (`c:\Users\jlaut\git\RetroGameGame`)

## Standard Commands

- Build: `cargo build`
- Build one binary: `cargo build --bin retro-game-game`
- Run: `cargo run --bin retro-game-game`
- Test: `cargo test`
- Check: `cargo check`
- Clippy: `cargo clippy --all-targets --all-features`
- Format gate: `cargo fmt -- --check`

## Branching Mandate

- Core branches are `develop` and `main`.
- Ticket branches must start from `develop` and use the `codex/` prefix.
- Ticket merges go to `develop`; only promoted, fully gated releases go to `main`.
- Principal engineer is the sole merger to `main`.

## Active Specialist Roster

- `principal_engineer`: scope, sequencing, product focus, merge decisions
- `Aarne Tasapaino`: gameplay loop, combat feel, progression, encounter rules
- `Pekka Kone`: engine/runtime, Bevy wiring, camera, movement, spawning, integration
- `Aino Kuvitus`: art + music direction, 2D concept sheets for 3D handoff, chapter music briefs
- `Ilmari Maasto`: plot/chapter direction, factions, enemy families, ground identity, chapter progression

## Non-Default Roles

- `Sanna Laatu`: release-only QA and HITL validation, not a mandatory step for normal tickets
- `Veikko Fiilis`: dormant specialist for VFX/audio implementation when explicitly activated

## Task Handoff Contract

- `agents/status/current_focus.md` is the single live source of truth for current work.
- Agent briefs define role boundaries and bootstrap rules.
- Create tickets only when work is implementation-ready.
- QA signoff artifacts are required only for release, milestone, or explicit HITL checkpoints.

## Model And Handoff Routing

- Every new implementation-ready ticket must declare:
  - `Complexity`: `Simple`, `Medium`, or `Complex`
  - `Recommended Specialist`: human-facing manual dispatch target, e.g. `Pekka Kone (Engine/Runtime)`
  - `Preferred Model`: `Gemini Flash`, `Gemini Pro`, or `Codex`
- `Owner Agent` remains the internal control-plane owner ID. `Recommended Specialist` tells the human who to hand the task to.
- Default routing:
  - `Simple` -> `Gemini Flash`
  - `Medium` -> `Codex`
  - `Complex` -> `Codex`
- Use `Gemini Pro` when the task is creative-first, graphical-design-heavy, or benefits from broader out-of-the-box ideation rather than precision implementation.
- Use `Codex` for troubleshooting, scoped implementation, instruction-following, and precision-sensitive engine/runtime work.
- `Opus` is escalation-only when other models are struggling and should not appear as a normal preferred-model value in ticket metadata.

## Art And Audio Direction

- Core 3D gameplay assets are human-created `.glb` models.
- Aino's planning lane focuses on 2D concept sprites, visual callouts, and chapter music briefs that support later 3D modeling and implementation.
- Do not treat old Nebula 2D sprite-generation docs as active pipeline guidance.

## Archive Policy

- Legacy prompts and superseded operational docs live outside the repo at `C:\Users\jlaut\agent-archive\RetroGameGame\`.
- Archived material is historical reference only and should not be part of normal bootstrap.
