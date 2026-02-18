# Local Agent Starter Prompts

These prompts are copy/paste starters for local Antigravity/Gemini agents.

Use one agent per prompt and enforce one-ticket WIP.

Personal workspace shortcuts (recommended launch surface):

- Aarne Tasapaino (`agent1`): `/home/jl/git/RetroGameGame/agents/team/aarne_tasapaino`
- Pekka Kone (`agent2`): `/home/jl/git/RetroGameGame/agents/team/pekka_kone`
- Ilmari Maasto (`agent3`): `/home/jl/git/RetroGameGame/agents/team/ilmari_maasto`
- Aino Kuvitus (`agent4`): `/home/jl/git/RetroGameGame/agents/team/aino_kuvitus`
- Veikko Fiilis (`agent5`): `/home/jl/git/RetroGameGame/agents/team/veikko_fiilis`
- Sanna Laatu (`qa`): `/home/jl/git/RetroGameGame/agents/team/sanna_laatu`

- Agent 1: `/home/jl/git/RetroGameGame/agents/prompts/agent1_start_prompt.md`
- Agent 1 (integration presets active): `/home/jl/git/RetroGameGame/agents/prompts/agent1_kickoff_nb-a1-003.md`
- Agent 2: `/home/jl/git/RetroGameGame/agents/prompts/agent2_start_prompt.md`
- Agent 2 (asset runtime integration active): `/home/jl/git/RetroGameGame/agents/prompts/agent2_kickoff_nb-a2-003.md`
- Agent 3: `/home/jl/git/RetroGameGame/agents/prompts/agent3_start_prompt.md`
- Agent 3 (procgen expansion active): `/home/jl/git/RetroGameGame/agents/prompts/agent3_kickoff_nb-a3-002.md`
- Agent 4: `/home/jl/git/RetroGameGame/agents/prompts/agent4_start_prompt.md`
- Agent 4 (core gameplay sprite pack active): `/home/jl/git/RetroGameGame/agents/prompts/agent4_kickoff_nb-a4-004.md`
- Agent 5: `/home/jl/git/RetroGameGame/agents/prompts/agent5_start_prompt.md`
- Agent 5 (hit-stop tuning active): `/home/jl/git/RetroGameGame/agents/prompts/agent5_kickoff_nb-a5-002.md`
- QA: `/home/jl/git/RetroGameGame/agents/prompts/qa_start_prompt.md`
- QA (art gate signoff active): `/home/jl/git/RetroGameGame/agents/prompts/qa_kickoff_nb-qa-010.md`
- Principal Engineer: `/home/jl/git/RetroGameGame/agents/principal_engineer/launch_prompt.md`

Shared constraints for all agents:

- Local-first execution only. Do not use Jules/cloud lane.
- Do not modify files outside ticket `Allowed Paths`.
- Return report file at the exact path in the delegation brief.
- If blocked, report `Status: BLOCKED` with concrete unblock request.
- For art tickets, avoid white-background source assets; prefer transparent/chroma-key workflows and run `assets/scripts/check_bg.py --strict`.
