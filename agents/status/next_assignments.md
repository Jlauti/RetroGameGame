# Next Assignments

Updated: 2026-02-25
Owner: principal_engineer

Use this as the single source of truth for the next work item per agent.

| Agent Name | Agent ID | Next Ticket | Lane | Ticket File | Delegation File | Report Target |
|---|---|---|---|---|---|---|
| Aarne Tasapaino | agent1 | NB-A1-006 | LOCAL | `c:\Users\jlaut\git\RetroGameGame/agents/backlog/NB-A1-006.md` | `c:\Users\jlaut\git\RetroGameGame/agents/delegations/agent1/NB-A1-006_task.md` | `c:\Users\jlaut\git\RetroGameGame/agents/reports/agent1/NB-A1-006_task_report.md` |
| Pekka Kone | agent2 | NB-A2-008 | LOCAL | `c:\Users\jlaut\git\RetroGameGame/agents/backlog/NB-A2-008.md` | `c:\Users\jlaut\git\RetroGameGame/agents/delegations/agent2/NB-A2-008_task.md` | `c:\Users\jlaut\git\RetroGameGame/agents/reports/agent2/NB-A2-008_task_report.md` |
| Ilmari Maasto | agent3 | NB-A3-003 | LOCAL | `c:\Users\jlaut\git\RetroGameGame/agents/backlog/NB-A3-003.md` | `c:\Users\jlaut\git\RetroGameGame/agents/delegations/agent3/NB-A3-003_task.md` | `c:\Users\jlaut\git\RetroGameGame/agents/reports/agent3/NB-A3-003_task_report.md` |
| Aino Kuvitus | agent4 | NB-A4-011 | LOCAL | `c:\Users\jlaut\git\RetroGameGame/agents/backlog/NB-A4-011.md` | `c:\Users\jlaut\git\RetroGameGame/agents/delegations/agent4/NB-A4-011_task.md` | `c:\Users\jlaut\git\RetroGameGame/agents/reports/agent4/NB-A4-011_task_report.md` |
| Veikko Fiilis | agent5 | NONE (idle) | LOCAL | n/a | n/a | n/a |
| Sanna Laatu | qa | NB-QA-020 (blocked) | LOCAL | `c:\Users\jlaut\git\RetroGameGame/agents/backlog/NB-QA-020.md` | `c:\Users\jlaut\git\RetroGameGame/agents/delegations/qa/NB-QA-020_task.md` | `c:\Users\jlaut\git\RetroGameGame/agents/reports/qa/NB-QA-020_task_report.md` |
| Codex Worker 1 | codex_worker1 | NB-CX-011 (merge-prep closeout) | LOCAL | `c:\Users\jlaut\git\RetroGameGame/agents/backlog/NB-CX-011.md` | `c:\Users\jlaut\git\RetroGameGame/agents/delegations/codex_worker1/NB-CX-011_task.md` | `c:\Users\jlaut\git\RetroGameGame/agents/reports/codex_worker1/NB-CX-011_task_report.md` |
| Codex Worker 2 | codex_worker2 | NONE (idle) | LOCAL | n/a | n/a | n/a |

## Notes

- One-ticket WIP per agent remains enforced.
- QA starts NB-QA-020 only after NB-A1-006, NB-A2-008, NB-A3-003, and NB-A4-011 are DONE.
- Graphics wave focus:
  1. lock behind-ship camera and topography contract
  2. produce deterministic topography data layer
  3. implement neon-hex topography runtime rendering
  4. validate readability and regressions in HITL
