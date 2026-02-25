# Next Cycle Plan - Post-HITL

## Mini Plan (Immediate)

1. Run HITL checklist from `c:\Users\jlaut\git\RetroGameGame/agents/status/hitl/HITL_round_2026-02-17.md`.
2. Triage findings into:
   - blocker: must-fix before merge promotion
   - major: merge-allowed only with explicit follow-up ticket
   - minor: backlog for next polish cycle
3. Finalize merge wave on `NB-CX-007..010` using:
   - `c:\Users\jlaut\git\RetroGameGame/agents/merge/merge_order_wave3_2026-02-17.md`

## Next Work Wave (After HITL)

1. Aino Kuvitus (`agent4`)
   - Continue `NB-A4-006` with gameplay-first sprite coverage:
     - player ship
     - enemy ship set
     - terrain/ground tiles
   - Deliver matching metadata per sprite (intended use, orientation basis, pivot note).
2. Codex Worker 1 (`codex_worker1`)
   - Integrate approved sprite assets into runtime spawn/render paths.
   - Add explicit fallback visuals if asset load fails.
3. Codex Worker 2 (`codex_worker2`)
   - Build/extend sprite inspection tooling + metadata index output for QA/HITL review.
   - Keep this isolated from gameplay logic changes.
4. Veikko Fiilis (`agent5`)
   - Implement screen-shake damping/tuning pass based on HITL results.
5. Sanna Laatu (`qa`)
   - Gate each completed ticket and publish canonical PASS/FAIL signoffs.

## Done Definition for This Cycle

1. HITL results recorded with reproducible evidence.
2. Merge candidates are deterministic and manifest-backed.
3. Asset production is directly tied to in-game integration and QA visibility.

