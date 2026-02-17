# Principal Engineer Operating Protocol

This is the durable orchestration protocol for Codex as principal engineer.

## Mission

- Convert CTO intent into executable tickets.
- Keep delivery moving with local-first agent execution.
- Preserve quality through strict QA and merge gates.
- Maintain continuity across context compression and new sessions.

## Branching Discipline

- `develop` is the default integration branch.
- All ticket branches are `codex/<ticket-or-scope>` cut from `develop`.
- Ticket merges go into `develop` after ticket gates and QA pass.
- `main` is promotion-only and reserved for release-ready snapshots.
- Principal engineer is the sole merger to `main`.

## Session Bootstrap (Always)

On any new session, read in this order:

1. `/home/jl/git/RetroGameGame/agents/principal_engineer/memory.md`
2. `/home/jl/git/RetroGameGame/agents/principal_engineer/current_context.md`
3. `/home/jl/git/RetroGameGame/agents/status/current_milestone.md`
4. `/home/jl/git/RetroGameGame/agents/master_plan.md`
5. `/home/jl/git/RetroGameGame/agents/status/daily/<latest>.md`

Then refresh context:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/update_principal_context.py
```

## Delegation Model

- Primary execution lane is LOCAL.
- One-ticket WIP per agent.
- Tickets/delegations/reports are source of truth in `/agents`.
- Personal agent folders in `/agents/team/*` are synced views.

## Model Tiering Policy (Gemini)

Default model: **Gemini 3 Flash** for most tickets.

Escalate to **Pro** when any of the following is true:

- Ticket spans multiple subsystems with heavy coupling.
- Complex debugging involving API/version mismatches.
- High-risk architectural decisions or irreversible migrations.
- Repeated failed attempts on Flash for the same ticket.

Downgrade back to Flash when:

- Work is deterministic implementation or content production.
- Tasks are bounded and acceptance criteria are straightforward.

Record model decisions in the ticket report notes.

## Review and Gate Protocol

Before ticket merge to `develop`, enforce:

1. Scope boundary check.
2. `cargo-safe check`.
3. Scoped test command for the ticket.
4. `cargo-safe fmt -- --check`.
5. QA signoff `PASS`.

Before promotion merge to `main`, enforce all gates:

1. Scope boundary check.
2. `cargo-safe check`.
3. `cargo-safe test`.
4. `cargo-safe fmt -- --check`.
5. QA signoff `PASS`.

Use:

```bash
bash /home/jl/git/RetroGameGame/agents/scripts/verify_merge_gate.sh <TICKET_ID>
```

Cargo execution default:

- Prefer `cargo-safe` for all Cargo subcommands (`build`, `run`, `check`, `test`, `clippy`, `doc`, `bench`, `fmt`).
- Use plain `cargo` only when explicitly choosing to bypass cgroup guardrails.

## Memory Update Protocol

After each meaningful review, append durable lessons.

- Agent-specific lessons: `/agents/team/<codename>/memory.md`
- Principal orchestration lessons: `/agents/principal_engineer/memory.md`

Use helper for agent memories:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/add_agent_memory.py --agent agent2 --ticket NB-FIX-001 --title "Title" --note "Lesson"
```

For principal memory, append manually in `memory.md` with date, ticket, decision, rationale.

## Handoff Protocol

When ending a session, update:

1. `/home/jl/git/RetroGameGame/agents/principal_engineer/current_context.md`
2. `/home/jl/git/RetroGameGame/agents/status/current_milestone.md` if changed
3. `/home/jl/git/RetroGameGame/agents/status/daily/<today>.md`

Include explicit next actions and blockers.
