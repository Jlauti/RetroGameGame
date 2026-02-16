# NB-CX-008 ProcGen Soak Harness Notes

## Simulation Inputs

Use `ProcgenSoakConfig` to run deterministic offline simulations with:

- `seed`: fixed RNG seed for reproducibility.
- `steps`: number of chunk picks to simulate.
- `start_profile`: starting top-edge profile expected by the first pick.
- `start_current_pacing`, `start_previous_pacing`, `start_chunks_in_current_pacing`: initial pacing state.

## Summary Outputs

`simulate_procgen_soak` returns `ProcgenSoakSummary` with:

- `pacing_pick_counts`: selected chunk counts by pacing.
- `chunk_pick_counts`: selected chunk counts by chunk name.
- `rejection_counts`: filtered-out candidates by reason (`profile_mismatch`, `softlock_constraint`, `invalid_weight`, `no_candidate`).
- `longest_same_pacing_streak`: longest consecutive run of the same selected pacing.
- `pacing_sequence` and `chunk_sequence`: deterministic pick trace for regression checks.
- `pacing_fallback_count`: number of picks where profile-matched fallback was used because target pacing had no candidates.

## Snapshot Export

To persist a snapshot for review:

- Use `write_default_procgen_soak_snapshot(&summary)` to write to:
  - `specs/future/nebula_bouncer/procgen_soak_snapshot_v1.json`
- Or use `write_procgen_soak_snapshot(&summary, custom_path)` for ad-hoc analysis output.

## Weight Rebalance Guidance

`rebalance_chunk_weights` proposes normalized chunk weights from observed drift:

- Input observed pick counts keyed by chunk name.
- Provide min/max per-chunk bounds (for example `0.05..0.50`).
- Output is normalized to sum to `1.0` and clamped within bounds when feasible.

Interpretation rule:

- If a chunk is under-selected relative to expected share, the proposal increases its share.
- If a chunk is over-selected, the proposal reduces its share.

## Guardrails For Future Chunk Additions

- Always define valid `top_profile` and `bottom_profile` so profile-matching does not collapse into excessive fallback.
- Keep chunk `weight` finite and strictly positive.
- Run softlock validation on new wall layouts before relying on soak data.
- Re-run deterministic soak with fixed seeds after adding/removing chunks to preserve comparable trend baselines.
