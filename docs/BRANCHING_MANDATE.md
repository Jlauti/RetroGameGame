# Branching Mandate

This repository uses a strict two-core-branch model:

- `main`: production-ready, human-tested baseline.
- `develop`: integration branch for validated ticket work.

All other branches are temporary ticket or integration branches and must be short-lived.

## Rules

1. Ticket branches branch from `develop`.
2. Ticket branches use the `codex/` prefix.
3. Ticket merges target `develop`, not `main`.
4. `main` updates only by explicit promotion from `develop`.
5. Principal engineer is the sole merger to `main`.

## Gate Requirements

Before merging a ticket branch into `develop`:

1. Scope boundary check passes.
2. Required ticket gates pass (`check`, scoped test, `fmt --check`).
3. QA signoff is `PASS`.

Before promoting `develop` into `main`:

1. Full merge gate passes.
2. QA confirms release candidate readiness.
3. Merge manifest is recorded.

## Daily Practice

Run branch normalization at session start:

```bash
bash /home/jl/git/RetroGameGame/agents/scripts/sync_core_branches.sh
```

This keeps local `main` synced to `origin/main` and ensures `develop` is tracking `origin/develop`.
