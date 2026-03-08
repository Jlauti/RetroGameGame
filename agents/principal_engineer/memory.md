# Principal Engineer Memory

Persistent orchestration memory for continuity across sessions.

## Working Preferences

- Keep orchestration local-first with strict ticket boundaries.
- Maintain one-ticket WIP per agent.
- Prefer fast unblock tickets over broad refactors.
- Each agent reads only `agents/INDEX.md` → their own `brief.md` → `memory.md`.

## Repeated Pitfalls

- Agent-reported completion can diverge from true gate readiness.
- New subsystem scaffolding can break compile gates if API mismatches are unchecked.
- Missing QA artifacts leads to false sense of completion.
- AI image generators do NOT produce real transparency — lesson learned from sprite pipeline.

## Proven Patterns

- Create dedicated compile-fix tickets immediately when build health drops.
- Maintain per-agent memory files to reduce re-onboarding overhead.
- Consolidated `brief.md` per agent eliminates confusion about which file to read.

## Review Notes

### Control Plane Simplification
- Recorded: 2026-03-07
- Context: The old loop accumulated too many overlapping "current state" artifacts and too much bootstrap noise.
- Decision: Active principal bootstrap now flows through `AGENTS.md`, `agents/INDEX.md`, `agents/principal_engineer/OPERATING_PROTOCOL.md`, and `agents/status/current_focus.md`.
- Reason: Keep Nebula-first delivery tight and prevent stale operational docs from competing with live status.

### Bootstrap Note
- Recorded: 2026-02-15
- Context: Established per-agent folders, personal memories, and milestone-driven control plane.
- Decision: Principal engineer protocol and memory must be first read in new sessions.

### Model Tiering Policy Confirmed
- Recorded: 2026-02-15
- Context: Team now routes work across Gemini Flash, Gemini Pro, and Codex, with manual handoff between harnesses.
- Decision:
  - every new ticket declares `Complexity`, `Recommended Specialist`, and `Preferred Model`
  - `Simple` defaults to `Gemini Flash`
  - `Medium` defaults to `Codex`
  - `Complex` defaults to `Codex`
  - `Gemini Pro` is used for creative-first, graphical, and out-of-the-box ideation tasks
  - `Codex` is used for troubleshooting, precision work, and scoped implementation
  - `Opus` is escalation-only and not part of normal ticket metadata
- Reason: Manual dispatch must make both the target specialist and the recommended harness obvious without opening additional docs.

### Gate Ownership Policy Confirmed
- Recorded: 2026-03-08
- Context: Worker agents and principal review were both running the same cargo gates, creating duplicate validation work.
- Decision:
  - implementation agents own ticket cargo gates and must report command/results in their task report
  - principal review uses reported gate results by default and does not rerun the same gates automatically
  - independent reruns happen only for explicit human request, release/HITL checkpoints, or when the agent report looks unreliable
- Reason: Keep validation cost down while preserving a clear override path when independent confirmation is actually needed.

### Windows Migration
- Recorded: 2026-02-21
- Context: Moved primary development from Linux laptop to Windows desktop for performance.
- Decision: All development now targets Windows. Key differences:
  - Python invoked via `py` (not `python3`)
  - Use plain `cargo` (not `cargo`)
  - Repo path: `c:\Users\jlaut\git\RetroGameGame`

### Agent Documentation Restructure
- Recorded: 2026-02-24
- Context: Agent folder had grown to 5-7 overlapping files per agent (nudge, context, memory, launch_prompt, README, backlog), causing confusion about what to read.
- Decision: Consolidated to 3 files per agent: `brief.md` (entry point), `memory.md` (persistent lessons), `inbox/suggestions.md` (evolution proposals). Created `agents/INDEX.md` as master map.
- Reason: Agents should read minimal, focused context — not everything.

### .glb Pipeline Shift
- Recorded: 2026-02-24
- Context: CTO decided core 2D and 3D assets will be human-created to maintain creative vision.
- Decision: Pipeline shifts from AI-generated sprites to human-created `.glb` models. Art agent (Aino Kuvitus) role changes from generation to integration/validation/consistency.
- Reason: Quality and consistency of creative direction outweigh AI generation speed.
- Action: All NB-A4-xxx sprite tickets archived. Art pipeline docs updated.

### Memory Suggestion Protocol
- Recorded: 2026-02-24
- Context: Agents had no mechanism to evolve their knowledge across tasks.
- Decision: After each task, agents append proposals to `inbox/suggestions.md`. Principal engineer reviews and curates. Approved items merge into `memory.md`, declined items noted and cleared.
- Reason: Selective evolution through proposals — not blind accumulation.
