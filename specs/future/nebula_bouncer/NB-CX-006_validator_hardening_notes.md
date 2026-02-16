# NB-CX-006 ProcGen Validator Hardening Notes

## Rejection Reason Taxonomy

- `profile_mismatch`: candidate `top_profile` does not match required incoming profile.
- `concave_trap`: nearby wall pair forms a deep near-right-angle pocket likely to trap ricochet paths.
- `exit_angle_fail`: nearby wall pair has an angle below minimum viable exit angle.

## Policy Thresholds

- `min_exit_angle_radians`: `0.55` (~31.5 degrees)
- `exit_angle_check_distance`: `150.0`
- `concave_trap_max_distance`: `110.0`
- `concave_right_angle_tolerance_radians`: `0.22`

## Runtime Integration

- Chunk selection runs validator checks before accepting a candidate.
- Invalid chunks are skipped, and rejection counters accumulate by reason.
- Preflight runs at startup across full chunk library and writes an artifact report.

## Preflight Artifact

- Output path:
  - `agents/deliverables/codex_worker2/NB-CX-006_preflight_summary.txt`
- Generated from setup with deterministic summary formatting.

## Example Preflight Summary Output

```text
Procgen Preflight Summary
total_chunks: 6
valid_chunks: 5
invalid_chunks: 1
rejections.profile_mismatch: 0
rejections.concave_trap: 1
rejections.exit_angle_fail: 0
```
