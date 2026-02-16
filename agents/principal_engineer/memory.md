# Principal Engineer Memory

Persistent orchestration memory for continuity across sessions.

## Working Preferences

- Keep orchestration local-first with strict ticket boundaries.
- Maintain one-ticket WIP per agent.
- Prefer fast unblock tickets over broad refactors.

## Repeated Pitfalls

- Agent-reported completion can diverge from true gate readiness.
- New subsystem scaffolding can break compile gates if API mismatches are unchecked.
- Missing QA artifacts leads to false sense of completion.

## Proven Patterns

- Create dedicated compile-fix tickets immediately when build health drops.
- Maintain per-agent memory/context files to reduce re-onboarding overhead.
- Regenerate daily digest after status changes to keep one truth source.

## Review Notes

### Bootstrap Note
- Recorded: 2026-02-15
- Context: Established per-agent folders, personal memories, and milestone-driven control plane.
- Decision: Principal engineer protocol and memory must be first read in new sessions.

### Model Tiering Policy Confirmed
- Recorded: 2026-02-15
- Context: Team now has Gemini 3 Flash and optional Pro access.
- Decision: Default to Flash; escalate selected hard/debug/architecture tickets to Pro.
- Reason: Maintain throughput while reserving higher intelligence for high-complexity tasks.

### Cargo Guardrail Policy
- Recorded: 2026-02-15
- Context: Long Bevy compile/test runs can hit memory pressure and end abruptly.
- Decision: Default all agent compile/test/build flows to `cargo-safe`; plain `cargo` requires explicit intent.
- Reason: Reduce OOM-style interruptions and stabilize merge/QA gate execution.
