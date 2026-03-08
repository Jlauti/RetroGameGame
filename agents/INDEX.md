# Agent Control Plane

This folder contains the active control plane for Nebula-first delivery.

## Read Order

Every active agent reads only:

1. `AGENTS.md`
2. `agents/PRINCIPLES.md`
3. their own `agents/team/<codename>/brief.md`
4. `agents/status/current_focus.md`
5. the specific ticket/spec files linked from the brief or assignment

Do not read other agent briefs, archived prompts, or broad process docs unless the current task explicitly requires them.

## Live Source Of Truth

- `agents/status/current_focus.md`: current objective, ownership, blockers, acceptance, release checkpoint notes

## Active Roles

| Name | Role | Default Status |
|------|------|----------------|
| Principal Engineer | Scope, sequencing, merge decisions, product focus | Active |
| Aarne Tasapaino | Gameplay loop, combat feel, progression, encounters | Active |
| Pekka Kone | Engine/runtime, Bevy integration, camera, movement, spawning | Active |
| Aino Kuvitus | Art + music direction, 2D concept sheets, chapter music briefs | Active |
| Ilmari Maasto | Plot/chapter direction, factions, ground identity, chapter progression | Active |
| Sanna Laatu | Release-only QA and HITL validation | On demand |
| Veikko Fiilis | VFX/audio implementation specialist | Dormant |

## Folder Map

| Folder | Purpose | Read If... |
|--------|---------|------------|
| `team/<codename>/` | Stable role brief, memory, suggestion inbox | It is your role |
| `status/` | Current focus and lightweight status artifacts | You need current work state |
| `backlog/` | Implementation-ready scoped tickets | `current_focus.md` points at a ticket |
| `principal_engineer/` | Principal kickoff and protocol | You are the principal engineer |
| `deliverables/` | Optional supporting notes for handoff-heavy work | A current task links to one |
| `reports/` | Optional historical reports | A current task links to one |
| `qa/` | Release-only QA artifacts | Release/HITL validation is active |
| `archive/` | In-repo historical material still kept locally | You were explicitly asked to inspect history |

## Working Rules

- Keep context clean and role-local.
- Do not create parallel status files that restate current ownership.
- Use tickets when work is ready for implementation, not as default bookkeeping.
- Prefer short role briefs over task-specific prompt sprawl.
- Use the suggestion inboxes for durable lessons, not for task status.
