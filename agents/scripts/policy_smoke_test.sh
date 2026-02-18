#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

make_ticket() {
  local path="$1"
  local lane="$2"
  local critical="$3"
  local eligible="$4"
  local owner="$5"
  local status="$6"

  cat > "$path" <<TXT
# Ticket

## Metadata

- Ticket ID: TMP-001
- Owner Agent: ${owner}
- Status: ${status}
- Execution Lane: ${lane}
- Critical Path: ${critical}
- Jules Eligible: ${eligible}
- Fallback Owner: ${owner}

## Allowed Paths

- src/
TXT
}

# Scenario 1: critical path + JULES is rejected.
make_ticket "$TMP_DIR/critical_jules.md" "JULES" "YES" "YES" "agent1" "TODO"
if python3 "$ROOT/agents/scripts/validate_ticket.py" --ticket "$TMP_DIR/critical_jules.md"; then
  echo "FAILED: critical-path JULES ticket should be rejected" >&2
  exit 1
fi

# Scenario 2: JULES without eligibility is rejected.
make_ticket "$TMP_DIR/jules_ineligible.md" "JULES" "NO" "NO" "agent1" "TODO"
if python3 "$ROOT/agents/scripts/validate_ticket.py" --ticket "$TMP_DIR/jules_ineligible.md"; then
  echo "FAILED: ineligible JULES ticket should be rejected" >&2
  exit 1
fi

# Scenario 3: local ticket is accepted.
make_ticket "$TMP_DIR/local_ok.md" "LOCAL" "YES" "NO" "agent1" "TODO"
python3 "$ROOT/agents/scripts/validate_ticket.py" --ticket "$TMP_DIR/local_ok.md"

# Scenario 4: one-ticket WIP violations are detected.
mkdir -p "$TMP_DIR/backlog"
make_ticket "$TMP_DIR/backlog/a.md" "LOCAL" "YES" "NO" "agent2" "IN_PROGRESS"
make_ticket "$TMP_DIR/backlog/b.md" "LOCAL" "YES" "NO" "agent2" "IN_PROGRESS"
if python3 "$ROOT/agents/scripts/check_wip.py" --backlog "$TMP_DIR/backlog"; then
  echo "FAILED: WIP violation should be detected" >&2
  exit 1
fi

# Scenario 5: QA signoff blocks non-PASS.
cat > "$TMP_DIR/qa_fail.md" <<TXT
# QA Signoff

## Metadata

- Ticket ID: TMP-001
- QA Engineer: qa
- Date: 2026-02-15
- Gate Result: FAIL
TXT

if python3 "$ROOT/agents/scripts/check_qa_signoff.py" --ticket-id TMP-001 --qa-file "$TMP_DIR/qa_fail.md"; then
  echo "FAILED: QA FAIL should block merge gate" >&2
  exit 1
fi

# Scenario 6: docs-only non-throughput loop is rejected.
cat > "$TMP_DIR/loop_docs_only.md" <<TXT
# Loop

## Metadata

- Loop ID: LOOP-TEST-001
- Name: Docs Loop
- Owner: principal_engineer
- Status: ACTIVE
- Value Hypothesis: Improve docs.
- Value Class: GAMEPLAY

## Scope In

- docs/agentic/
- agents/status/

## Scope Out

- src/

## Tickets Included

- NB-CX-011

## Worker Plan

- principal_engineer: orchestration

## Acceptance Commands

- cargo-safe fmt -- --check

## Acceptance Evidence Required

- docs updated

## Completion Gate

- docs shipped
TXT

if python3 "$ROOT/agents/scripts/validate_loop.py" --loop "$TMP_DIR/loop_docs_only.md"; then
  echo "FAILED: docs-only non-throughput loop should be rejected" >&2
  exit 1
fi

# Scenario 7: gameplay loop with runtime scope is accepted.
cat > "$TMP_DIR/loop_gameplay_ok.md" <<TXT
# Loop

## Metadata

- Loop ID: LOOP-TEST-002
- Name: Gameplay Loop
- Owner: principal_engineer
- Status: ACTIVE
- Value Hypothesis: Improve gameplay.
- Value Class: GAMEPLAY

## Scope In

- src/eras/era_future/nebula_bouncer/

## Scope Out

- docs/

## Tickets Included

- NB-CX-011

## Worker Plan

- principal_engineer: orchestration
- agent2: implementation

## Acceptance Commands

- cargo-safe check --bin retro-game-game

## Acceptance Evidence Required

- QA PASS

## Completion Gate

- QA PASS
TXT

python3 "$ROOT/agents/scripts/validate_loop.py" --loop "$TMP_DIR/loop_gameplay_ok.md"

echo "Policy smoke test passed."
