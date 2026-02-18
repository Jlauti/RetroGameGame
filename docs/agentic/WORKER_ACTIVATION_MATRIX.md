# Worker Activation Matrix

## Principal Engineer

- Always active per loop.
- Responsibilities:
  - define loop artifact
  - assign tickets and lanes
  - queue gates and merges
  - finalize completion decision

## Domain Agents (`agent1`..`agent5`)

Activate when:

- ticket domain matches agent role
- ticket status is actionable (`TODO` or `IN_PROGRESS` by assignment)
- one-ticket WIP policy remains satisfied

## QA (`qa`)

Activate when:

- ticket reaches `READY_FOR_QA`
- loop completion checkpoint is running

## Codex Workers (`codex_worker1`, `codex_worker2`)

Activate when:

- loop has at least two parallelizable technical tickets, or
- throughput risk indicates merge-ready backlog is growing faster than principal merge capacity

## Jules (Optional)

Activate only when both are true:

- `Jules Eligible: YES`
- `Critical Path: NO`

And fail-fast safeguards are enforced:

- no usable plan in 10 min -> unhealthy
- no usable patch in 30 min after plan approval -> unhealthy
- maximum one retry before local takeover
