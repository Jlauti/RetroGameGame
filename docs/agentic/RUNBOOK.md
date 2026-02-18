# Principal Runbook

## Session Start

1. Read:
   - `agents/principal_engineer/memory.md`
   - `agents/principal_engineer/current_context.md`
   - `agents/principal_engineer/OPERATING_PROTOCOL.md`
2. Refresh context:
   - `python3 agents/scripts/update_principal_context.py`
3. Validate active loop artifact:
   - `python3 agents/scripts/validate_loop.py --loop agents/loops/<LOOP_ID>.md`

## Merge Throughput Cycle

1. Refresh readiness + queue merge-ready work:
   - `python3 agents/scripts/queue_merge_ready.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
2. Keep gate runner active:
   - `python3 agents/scripts/gate_queue.py run-loop --wait-lock`
3. Reconcile after merge wave:
   - `python3 agents/scripts/reconcile_ticket_state.py --root /home/jl/git/RetroGameGame --date $(date +%F)`
   - `python3 agents/scripts/build_release_board.py --root /home/jl/git/RetroGameGame --date $(date +%F)`

## End Of Day

1. Refresh principal context.
2. Confirm loop status (`ACTIVE`, `BLOCKED`, or `COMPLETE`).
3. Ensure next actions are explicit in the loop artifact and principal context.
