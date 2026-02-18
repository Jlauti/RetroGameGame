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

echo "Policy smoke test passed."
