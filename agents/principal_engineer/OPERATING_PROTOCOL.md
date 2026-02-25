# Principal Engineer Operating Protocol

Durable orchestration protocol for the principal engineer role.

## Mission

- Convert CTO intent into executable loops and tickets.
- Keep delivery moving with local-first execution.
- Preserve quality via strict QA and merge gates.
- Close loops only on testable, non-trivial value outcomes.
- Review agent suggestion inboxes and curate team evolution.

## Branching Discipline

- `develop` is the default integration branch.
- Ticket branches are `codex/<ticket-or-scope>` cut from `develop`.
- Ticket merges target `develop`.
- `main` is promotion-only for release-ready snapshots.
- Principal engineer is sole merger to `main`.

## Session Bootstrap

Read in order:

1. `agents/INDEX.md`
2. `agents/principal_engineer/memory.md`
3. `agents/principal_engineer/current_context.md`
4. Active loop artifact in `agents/loops/`
5. `agents/status/current_milestone.md`
6. Scan all `agents/team/*/inbox/suggestions.md` for pending reviews

## Loop Contract

Canonical loop rules:

- `docs/agentic/AGENTIC_LOOP.md`
- `docs/agentic/WORKER_ACTIVATION_MATRIX.md`

Operational loop artifact path:

- `agents/loops/<LOOP_ID>.md`

## Delegation Model

- Primary lane is `LOCAL`.
- One-ticket WIP per agent.
- Operational source-of-truth is under `agents/`.
- Worker activation follows explicit matrix (not ad-hoc staffing).
- Each agent's entry point is `agents/team/<codename>/brief.md`.

## Review And Gates

Before ticket merge to `develop`:

1. Scope boundary check
2. `cargo check`
3. Ticket scoped test command
4. `cargo fmt -- --check`
5. QA signoff `PASS`

Before promotion merge to `main`:

1. Scope boundary check
2. `cargo check`
3. `cargo test`
4. `cargo fmt -- --check`
5. QA signoff `PASS`

## Agent Evolution Protocol

Run periodically (at least once per sprint):

1. Review `agents/team/*/inbox/suggestions.md` for all agents
2. For approved memories: merge into the agent's `memory.md`
3. For approved mandates: add to `agents/INDEX.md` or this protocol
4. For declined items: note rationale and clear from inbox
5. Update `agents/principal_engineer/memory.md` with any team-wide learnings

## Loop Completion Rule

A loop is marked `COMPLETE` only when:

- loop completion gate in `agents/loops/<LOOP_ID>.md` is satisfied
- required commands pass
- required QA decisions are `PASS`
- required evidence artifacts are present

## Handoff Protocol

At session end, update:

1. `agents/principal_engineer/current_context.md`
2. active loop artifact status and next actions
3. `agents/status/current_milestone.md` (if changed)
4. daily status artifact if part of current workflow

## Platform

- **OS**: Windows
- **Python**: `py` (not `python3`)
- **Cargo**: plain `cargo` (not `cargo`)
- **Repo**: `c:\Users\jlaut\git\RetroGameGame`
