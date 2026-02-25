# Agent Control Plane — Master Index

Every agent session starts here. This document maps the folder structure and tells you **exactly what to read** based on your role.

## Quick Start

1. **Read this file** (you're doing it)
2. **Read the project soul** → `agents/PRINCIPLES.md` (project intent + 5 core principles)
3. **Find your agent brief** → `agents/team/<your_codename>/brief.md`
4. **Read your memory** → `agents/team/<your_codename>/memory.md`
5. **Execute your current task** as directed in the brief
6. **After task completion**, append suggestions to `agents/team/<your_codename>/inbox/suggestions.md`

> [!IMPORTANT]
> Do NOT read every .md file in the agents folder. Read only what your `brief.md` tells you to read plus this INDEX.

---

## Folder Map

| Folder | What's Inside | Read If... |
|--------|--------------|------------|
| `team/<codename>/` | Your identity, memory, and suggestion inbox | **Always** (your own folder only) |
| `backlog/` | Ticket definitions (scope, acceptance, deliverables) | Your `brief.md` references a specific ticket |
| `delegations/<agentN>/` | Detailed task briefs per ticket | Your `brief.md` references a delegation |
| `deliverables/<agentN>/` | Completed work output | You need to reference prior work |
| `reports/<agentN>/` | Post-task completion reports | You need to understand what was already done |
| `qa/` | QA signoff artifacts | You are the QA agent (Sanna Laatu) |
| `merge/` | Merge manifests and checklists | You are the principal engineer |
| `status/` | Gates, milestones, release boards | You are the principal engineer |
| `loops/` | Active execution loop contracts | You are the principal engineer |
| `scripts/` | Automation scripts (Python/Bash) | You need to run a validation or gate |
| `archive/` | Superseded tickets, old prompts, retired docs | Almost never — historical reference only |
| `principal_engineer/` | PE memory, protocol, context | You are the principal engineer |

---

## Team Roster

| Codename | Agent ID | Role |
|----------|----------|------|
| Aarne Tasapaino | agent1 | Gameplay Balance & Systems Design |
| Pekka Kone | agent2 | Runtime Integration & Engine Wiring |
| Ilmari Maasto | agent3 | Level/Terrain Design & Procedural Generation |
| Aino Kuvitus | agent4 | Asset Integration & Visual Consistency |
| Veikko Fiilis | agent5 | Audio, VFX & Game Feel |
| Sanna Laatu | qa | Quality Assurance & Gate Validation |

---

## Memory Suggestion Protocol

After completing a task, every agent MUST append to their `inbox/suggestions.md`:

```markdown
### Suggestion — <date>
- **Type**: memory | mandate | observation
- **Summary**: <one-line summary>
- **Detail**: <what you learned and why it matters>
- **Proposed Action**: <what should be added to memory.md or team policy>
```

The principal engineer reviews inboxes periodically and curates:
- **Approved** → merged into agent's `memory.md` or `principal_engineer/memory.md`
- **Declined** → noted with brief rationale, entry removed from inbox

> [!TIP]
> Most suggestions will be declined. That's fine. The goal is evolution through selective pressure, not accumulation.

---

## Platform & Build Rules

- **OS**: Windows
- **Python**: Use `py` (not `python3`)
- **Cargo**: Use plain `cargo` (not `cargo` — that was for the old Linux machine)
- **Repo Root**: `c:\Users\jlaut\git\RetroGameGame`
- **Branching**: `develop` = integration, `codex/<scope>` = ticket branches, `main` = release-only

---

## Art Pipeline (Current)

- **Core 2D and 3D assets are human-created** — the CTO owns creative vision
- Pipeline: Human creates `.glb` models → placed in `assets/models/<era>/<game>/` → Bevy loads at runtime
- Art agent (Aino Kuvitus) role: asset integration, consistency validation, Bevy loading verification
- Art agent does **NOT** generate core gameplay assets

---

## Principal Engineer Session Bootstrap

Read in order:
1. This file (`agents/INDEX.md`)
2. `agents/principal_engineer/memory.md`
3. `agents/principal_engineer/current_context.md`
4. Active loop in `agents/loops/`
5. `agents/status/current_milestone.md`
6. Scan all `agents/team/*/inbox/suggestions.md` for pending reviews
