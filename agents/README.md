# Nebula Agent Loop v3

Local-first operating model for Nebula Bouncer vertical-slice delivery.

## Authority Model

- CTO sets priorities, scope, and tradeoffs.
- Principal engineer owns branch decomposition, integration, and all merges to `main`.
- Agents deliver scoped artifacts via task/report files.
- QA is a per-ticket merge gatekeeper.

## Branch Model

- `main`: stable release branch, updated only through promotion from `develop`.
- `develop`: integration branch for validated ticket work.
- `codex/*`: short-lived ticket branches that branch from `develop`.
- Ticket merges target `develop`; release promotions target `main`.

## Control Plane

All orchestration artifacts live under `/home/jl/git/RetroGameGame/agents`.

- Agent roster and personal folders: `/home/jl/git/RetroGameGame/agents/team_roster.md`
- Personal workspaces root: `/home/jl/git/RetroGameGame/agents/team/`
- Principal engineer workspace: `/home/jl/git/RetroGameGame/agents/principal_engineer/`
- Master milestone plan: `/home/jl/git/RetroGameGame/agents/master_plan.md`
- Current milestone pointer: `/home/jl/git/RetroGameGame/agents/status/current_milestone.md`
- Backlog tickets: `/home/jl/git/RetroGameGame/agents/backlog/<ticket_id>.md`
- Delegation inputs: `/home/jl/git/RetroGameGame/agents/delegations/<agent_id>/<ticket_id>_task.md`
- Agent outputs: `/home/jl/git/RetroGameGame/agents/reports/<agent_id>/<ticket_id>_task_report.md`
- QA signoff: `/home/jl/git/RetroGameGame/agents/qa/<ticket_id>_qa_signoff.md`
- Merge manifest: `/home/jl/git/RetroGameGame/agents/merge/<ticket_id>_merge_manifest.md`
- Daily digest: `/home/jl/git/RetroGameGame/agents/status/daily/<YYYY-MM-DD>.md`

## Execution Policy

- Default execution lane is `LOCAL`.
- One-ticket WIP per agent.
- `JULES` is optional and experimental only.
- `JULES` can run only when `Jules Eligible: YES` and `Critical Path: NO`.
- Ineligible for `JULES`: balance math, boss logic, physics hooks, merge-blocking fixes, release blockers.

## Jules Fail-Fast Safeguards

- Unhealthy if no usable plan within 10 minutes.
- Unhealthy if no usable patch within 30 minutes after plan approval.
- Maximum one retry.
- If retry also unhealthy, immediate local takeover.
- Principal engineer may terminate any `JULES` run at any time.

## Merge Gates

Every ticket merge to `develop` must pass:

1. Scope boundary check.
2. Ticket gate set (`check`, scoped test, `fmt --check`).
3. QA signoff `PASS`.

Every promotion merge to `main` must pass:

1. Scope boundary check.
2. `cargo-safe check`.
3. `cargo-safe test`.
4. `cargo-safe fmt -- --check`.
5. QA signoff `PASS`.

Use `/home/jl/git/RetroGameGame/agents/scripts/verify_merge_gate.sh`.

## Cargo Execution Policy

- Default for agents: use `cargo-safe` for compile/test/build/run/lint/doc commands.
- Plain `cargo` is allowed only when intentionally bypassing guardrails.
- Per-run overrides for heavy tasks:
  - `MEM_HIGH=11G MEM_MAX=12G cargo-safe build`
  - `CARGO_BUILD_JOBS=6 cargo-safe test`
  - `MEM_HIGH=11G MEM_MAX=12G CARGO_BUILD_JOBS=6 cargo-safe build`

## Reporting Cadence

- Daily automation at 9:00 AM local generates the digest.
- Missing daily report for active tickets is an SLA breach.
- Manual nudges are used for blockers and SLA misses.

Generate manually:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/generate_daily_status.py
```

## Templates and Tooling

Templates:

- `/home/jl/git/RetroGameGame/agents/templates/ticket.md`
- `/home/jl/git/RetroGameGame/agents/templates/delegation_task.md`
- `/home/jl/git/RetroGameGame/agents/templates/task_report.md`
- `/home/jl/git/RetroGameGame/agents/templates/qa_signoff.md`
- `/home/jl/git/RetroGameGame/agents/templates/merge_manifest.md`

Starter prompts for local agent launches:

- `/home/jl/git/RetroGameGame/agents/prompts/agent1_start_prompt.md`
- `/home/jl/git/RetroGameGame/agents/prompts/agent2_start_prompt.md`
- `/home/jl/git/RetroGameGame/agents/prompts/agent3_start_prompt.md`
- `/home/jl/git/RetroGameGame/agents/prompts/agent4_start_prompt.md`
- `/home/jl/git/RetroGameGame/agents/prompts/agent5_start_prompt.md`
- `/home/jl/git/RetroGameGame/agents/prompts/qa_start_prompt.md`

Validation:

- `/home/jl/git/RetroGameGame/agents/scripts/sync_core_branches.sh`
- `/home/jl/git/RetroGameGame/agents/scripts/validate_ticket.py`
- `/home/jl/git/RetroGameGame/agents/scripts/check_wip.py`
- `/home/jl/git/RetroGameGame/agents/scripts/check_ticket_scope.py`
- `/home/jl/git/RetroGameGame/agents/scripts/check_qa_signoff.py`
- `/home/jl/git/RetroGameGame/agents/scripts/sync_agent_workspaces.py`
- `/home/jl/git/RetroGameGame/agents/scripts/add_agent_memory.py`
- `/home/jl/git/RetroGameGame/agents/scripts/update_principal_context.py`

## Per-Agent Folder Workflow

Use this when launching local agents via Antigravity/Gemini:

1. Run `python3 /home/jl/git/RetroGameGame/agents/scripts/sync_agent_workspaces.py`.
2. Open the agent's folder from `/home/jl/git/RetroGameGame/agents/team/`.
3. Agent reads:
   - `README.md`
   - `memory.md`
   - `context.md`
   - `backlog.md`
   - `inbox/<ticket>.md`
   - `launch_prompt.md`
4. Agent writes outputs to canonical `/agents/reports/...` paths.

## Memory Workflow

After reviewing an agent's work, append durable context into their personal memory file:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/add_agent_memory.py \
  --agent agent2 \
  --ticket NB-FIX-001 \
  --title \"Avian API pitfall\" \
  --note \"Use supported Bevy/Avian event/system APIs; avoid deprecated hierarchy imports.\"
```

Principal engineer continuity files:

- Protocol: `/home/jl/git/RetroGameGame/agents/principal_engineer/OPERATING_PROTOCOL.md`
- Memory: `/home/jl/git/RetroGameGame/agents/principal_engineer/memory.md`
- Current context snapshot: `/home/jl/git/RetroGameGame/agents/principal_engineer/current_context.md`

Refresh principal context snapshot:

```bash
python3 /home/jl/git/RetroGameGame/agents/scripts/update_principal_context.py
```
