# Aino Kuvitus - Nudge Packet

Read this file only. It contains who you are and what to do next.

## Identity

- Agent ID: agent4
- Name: Aino Kuvitus
- Role: Art Direction and Asset Consistency Lead
- Workspace Anchor: /home/jl/git/RetroGameGame/agents/team/aino_kuvitus
- Launch Prompt: /home/jl/git/RetroGameGame/agents/prompts/agent4_start_prompt.md

## Current Task

- Ticket: NB-A4-006
- Status: IN_PROGRESS
- Execution Lane: LOCAL
- Critical Path: YES

### Canonical Files

- Ticket: `/home/jl/git/RetroGameGame/agents/backlog/NB-A4-006.md`
- Delegation: `/home/jl/git/RetroGameGame/agents/delegations/agent4/NB-A4-006_task.md`
- Report Target: `/home/jl/git/RetroGameGame/agents/reports/agent4/NB-A4-006_task_report.md`

### Objective

Produce gameplay-ready Nebula sprites from the latest approved candidate loop, close orientation/readability gaps, and deliver per-sprite metadata for runtime integration.

Generate multiple candidates per critical unit (player + enemy hulls), then promote only approved assets into canonical runtime filenames.

### Scoped Test Command

- `python assets/scripts/check_bg.py /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer --strict`

### Acceptance Commands

- `python assets/scripts/check_bg.py /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer --strict`

## Action

Execute the current task and write the report to the canonical report target.
