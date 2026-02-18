# Agentic Loop Specification

## Loop Unit

The canonical loop unit is a **milestone loop**. A loop may contain one ticket or multiple tickets.

## Loop Requirements

Each loop must define:

- clear scope (`scope_in`, `scope_out`)
- value hypothesis (non-trivial expected value)
- included tickets
- worker plan
- acceptance commands
- required evidence artifacts
- completion gate conditions

Canonical artifact path:

- `agents/loops/<LOOP_ID>.md`

Template:

- `agents/templates/loop.md`

Validator:

- `python3 agents/scripts/validate_loop.py --loop agents/loops/<LOOP_ID>.md`

## Non-Trivial Value Rule

A loop must produce at least one of:

- gameplay/runtime quality improvement
- visual/assetization improvement in-game
- stability/performance/reliability improvement
- integration/merge throughput improvement

Docs-only or bookkeeping-only loops are invalid unless the loop class is explicitly `RELEASE_THROUGHPUT` and includes measurable merge/gate throughput outcomes.

## Completion Gate

A loop can be set to `COMPLETE` only when:

1. Included tickets satisfy declared loop outcomes.
2. Required gate commands are passing.
3. Required QA decisions are present.
4. Required evidence artifacts are present.
5. Principal engineer records final completion decision.
