# RetroGameGame

RetroGameGame is a multi-era arcade project built in Rust/Bevy.

## Agentic Loop At A Glance

Development is run as an **agentic loop**:

1. Define one non-trivial loop scope with explicit value target.
2. Assign one or more tickets to the right workers.
3. Execute work in parallel where safe.
4. Validate with gates and QA.
5. Merge non-conflicting ready work to `develop`.
6. Close the loop only when acceptance evidence is complete.

A loop can run with one worker or several workers depending on scope, but every loop must produce significant project value (gameplay quality, visual quality, stability, or delivery throughput), not only trivial docs churn.

## Workers And Activation Conditions

| Worker | Activates When |
|---|---|
| Principal Engineer | Always active; owns scope, dispatch, merge sequencing, and completion decision |
| `agent1`..`agent5` | Domain-matched ticket exists and one-ticket WIP constraints are satisfied |
| `qa` | Ticket enters `READY_FOR_QA` and at loop completion checkpoint |
| `codex_worker1/2` | Loop has parallelizable technical backlog or integration throughput risk |
| `JULES` (optional) | Ticket is `Jules Eligible: YES` and `Critical Path: NO` |

Full rule matrix: `docs/agentic/WORKER_ACTIVATION_MATRIX.md`.

## Loop Completion Criteria

A loop is complete only when all are true:

- Scope in `agents/loops/<loop_id>.md` is satisfied.
- Required acceptance commands pass.
- Required QA and merge gates are satisfied.
- Required evidence artifacts are present.
- Principal engineer marks loop `COMPLETE`.

Loop contract details: `docs/agentic/AGENTIC_LOOP.md`.

## Where To Start

- Build/runtime guardrails: `AGENTS.md`
- Agent control plane: `agents/README.md`
- Agentic docs index: `docs/agentic/README.md`
- Machine switch protocol: `docs/agentic/MACHINE_SWITCH.md`
- Remote Rust compiler runbook: `docs/REMOTE_RUST_COMPILER.md`
- Principal launch prompt: `agents/principal_engineer/launch_prompt.md`

## Build Quickstart

```bash
cargo-safe check
cargo-safe test
cargo-safe run --bin retro-game-game
```

## BRP MCP Smoke Test

After building the game binary, run:

```bash
./scripts/test_bevy_brp_mcp.sh
```

This verifies:
- Codex MCP server `bevy-brp` is registered
- game boots with `BEVY_BRP_ENABLE=1`
- BRP endpoint on `127.0.0.1:15702` responds to JSON-RPC

For a Nebula-specific MCP visual tuning probe:

```bash
./scripts/mcp_nebula_visual_round.sh
```

Optional live mutation during probe:

```bash
MUTATE_BLOOM=1 BLOOM_INTENSITY=0.34 ./scripts/mcp_nebula_visual_round.sh
```
