# Stash Audit - 2026-02-18

## Scope

Audit of recovered worker-worktree stashes after consolidating to `/home/jl/git/RetroGameGame` on `develop`.

Source map: `/tmp/retrogame_worktree_stash_map_20260218_232006.tsv`

## Inputs Reviewed

- `stash@{0}` `87500646958676bbea180cb692d6d7f6053c7467` (`NB-A5-005` worker hedge)
- `stash@{1}` `97931dd5557f4530a4870df687d00181f4ca5ce6` (`NB-CX-006` worker report-only)
- `stash@{2}` `f4b7caf8aecfdf4684d124ac98fbb895032fa55c` (`NB-CX-004` synergy branch WIP)
- `stash@{3}` `8a0b5c7f9a05cf3eef1015553ba1b6dcddedc537` (`NB-CX-002` procgen branch WIP)
- `stash@{4}` `0049733415a125478e822a57c83a4454f083ad74` (`NB-CX-007` worker report-only)
- `stash@{5}` `d934de584261ac285ee8622a70268fe526d69c11` (`NB-CX-003` boss runtime WIP)
- `stash@{6}` `bddaeab1bef073c5d8bcc00981696dafa0305782` (full detached workspace backup)

## Classification

1. `stash@{0}`: `STALE_OBSOLETE`
- Content: alternative camera shake toggle path (`ScreenShakeEnabled`, F10 gate at `apply_shake`).
- Evidence: `develop` already contains `F10` camera-shake toggle via `CameraFeedbackSettings` in `src/eras/era_future/nebula_bouncer/systems.rs`.
- Decision: drop after audit.

2. `stash@{1}`: `EVIDENCE_ONLY`
- Content: `agents/reports/codex_worker2/NB-CX-006_task_report.md` (no code).
- Decision: restored report into working tree; drop stash.

3. `stash@{2}`: `SUPERSEDED_BY_INTEGRATED_WORK`
- Content: early synergy matrix/runtime wiring (`components/resources/systems/mod`) plus snapshot JSON.
- Evidence: `develop` already contains `OrbSynergyMatrix`, loadout, and synergy application paths from integrated CX runtime sequence.
- Decision: drop after audit.

4. `stash@{3}`: `SUPERSEDED_BY_INTEGRATED_WORK`
- Content: early procgen deterministic soak/selection/validation edits.
- Evidence: `develop` already contains deterministic soak and pacing policy (`determine_target_pacing`) and validator flow in `procgen.rs`.
- Decision: drop after audit.

5. `stash@{4}`: `EVIDENCE_ONLY`
- Content: `agents/reports/codex_worker1/NB-CX-007_task_report.md` (no code).
- Decision: restored report into working tree; drop stash.

6. `stash@{5}`: `UNIQUE_VALID_PENDING_TICKET`
- Content: boss runtime state machine draft (`Boss*` components/resources/systems) + `NB-CX-003` report/notes.
- Evidence: no boss runtime symbols currently live on `develop`.
- Decision: keep stash for follow-up ticketized integration review.

7. `stash@{6}`: `SAFETY_SNAPSHOT`
- Content: broad detached-workspace backup (control-plane docs plus large auxiliary artifacts).
- Decision: keep temporarily as recovery checkpoint until main promotion cycle is complete.

## Restored Artifacts (This Pass)

- Control-plane docs from `stash@{6}^3`:
  - missing merge manifests (`NB-CX-005..008`)
  - missing QA signoffs
  - status files (`current_milestone`, `dispatch_now`, `codex_workers_next`, daily digests)
- Reports:
  - `agents/reports/codex_worker1/NB-CX-007_task_report.md`
  - `agents/reports/codex_worker2/NB-CX-006_task_report.md`
- Queue-history evidence:
  - `agents/status/gates/queue/history/20260218T210927Z_merge_NB-CX-007_261620_967634.json`
  - `agents/status/gates/queue/history/20260218T210927Z_merge_NB-CX-007_261620_967634_baseline_eval.json`
  - `agents/status/gates/queue/history/20260218T210927Z_merge_NB-CX-008_261621_967735.json`
  - `agents/status/gates/queue/history/20260218T210927Z_merge_NB-CX-008_261621_967735_baseline_eval.json`

## Follow-up

- Keep only two unresolved stashes after cleanup:
  - `NB-CX-003` boss runtime candidate stash
  - full workspace safety snapshot stash
- Any reuse of `NB-CX-003` stash must be ticketed and gated as fresh `develop` work, not directly popped into branch tip.
