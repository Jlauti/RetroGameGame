# Merge Manifest - NB-CX-008

## Ticket

- Ticket ID: NB-CX-008
- Title: ProcGen Deterministic Soak Harness and Weight Rebalancer
- Owner Agent: codex_worker2

## Branching

- Source Branch: codex/nb-cx-008-procgen-soak-harness
- Source Commit: 6a8934d
- Target Branch: main

## Lane and Takeover

- Execution Lane Used: LOCAL
- Jules Eligible: NO
- Jules Session Used: NO
- Takeover/Fallback Event: NONE

## Scope

- Allowed paths respected: YES
- Files changed:
  - src/eras/era_future/nebula_bouncer/procgen.rs
  - specs/future/nebula_bouncer/NB-CX-008_procgen_soak_harness_notes.md

## Gate Evidence

- Gate Mode: TICKET
- Gate Job ID: 20260216T204556Z_ticket_NB-CX-008_610532_756077
- Gate Artifact Path: /home/jl/git/RetroGameGame/agents/status/gates/queue/history/20260216T204556Z_ticket_NB-CX-008_610532_756077.json
- Gate Status: PASS

## QA

- QA Signoff File: /home/jl/git/RetroGameGame/agents/qa/NB-CX-008_qa_signoff.md
- QA Status: PASS

## Merge Gate

- Merge-mode gate required before merge to main: YES
- verify_merge_gate.sh run: WAIVED_BASELINE
- Merge Gate Mode: MERGE (develop consolidation queue)
- Merge Gate Job ID: 20260218T210927Z_merge_NB-CX-008_261621_967735
- Merge Gate Artifact Path: /home/jl/git/RetroGameGame/agents/status/gates/queue/history/20260218T210927Z_merge_NB-CX-008_261621_967735.json
- Baseline Waiver Artifact Path: /home/jl/git/RetroGameGame/agents/status/gates/queue/history/20260218T210927Z_merge_NB-CX-008_261621_967735_baseline_eval.json
- Baseline Waiver Result: only known failures
- Historical Staging Gate Job ID: 20260216T212034Z_merge_NB-CX-007_684278_834637
- Historical Staging Branch: codex/merge-cx007-cx008

## Ready Decision

- Ready for PR creation: YES
- Ready for merge to main now: YES (pending develop->main promotion)
- Merge Decision: PENDING_MAIN_PROMOTION

## PR Link

- Draft PR URL: https://github.com/Jlauti/RetroGameGame/pull/new/codex/nb-cx-008-procgen-soak-harness
