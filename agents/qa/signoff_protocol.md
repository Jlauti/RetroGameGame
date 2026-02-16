# QA Signoff Protocol: Nebula Bouncer

This document defines PASS/FAIL requirements for QA signoff tickets.

## 1. Environment Health

Default gate (for code-affecting tickets):

```bash
cargo-safe check
cargo-safe test
cargo-safe fmt -- --check
```

If a gate run is memory-constrained, QA may increase limits for that run:

```bash
MEM_HIGH=11G MEM_MAX=12G CARGO_BUILD_JOBS=6 cargo-safe test
```

For non-code deliverable tickets (docs/art-only), QA may evaluate deliverable quality separately, but must explicitly state whether global build health is PASS/FAIL/UNCHANGED in the signoff rationale.

## 2. Art Ticket Gate (Required)

For gameplay-facing art tickets (`agent4` sprite packs), QA must verify:

1. No white-background contamination in promoted assets.
2. Background/alpha gate passes:

```bash
source /home/jl/git/RetroGameGame/.venv/bin/activate
python /home/jl/git/RetroGameGame/assets/scripts/check_bg.py /home/jl/git/RetroGameGame/assets/sprites/future/nebula_bouncer --strict
```

3. In-game readability evidence exists in report notes:
- Distinct player/enemy silhouettes.
- Practical sprite dimensions and pivots documented.
- Ground/wall tiles suitable for repeated placement.

If any of the above fails, QA must mark `Gate Result: FAIL`.

## 3. The "No Red" Rule

No code-affecting ticket shall be marked PASS if required merge-gate commands return non-zero exit code.

## 4. Signoff Artifact Requirements

Each signoff file must include:

1. Ticket ID and Gate Result.
2. Evidence summary (commands and/or deliverable checks).
3. Clear PASS/FAIL rationale.
4. Any residual risks and next-owner recommendation.
