#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <TICKET_ID> [BASE_REF]" >&2
  exit 2
fi

TICKET_ID="$1"
BASE_REF="${2:-main}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TICKET_PATH="${ROOT}/agents/backlog/${TICKET_ID}.md"
QA_PATH="${ROOT}/agents/qa/${TICKET_ID}_qa_signoff.md"

python3 "${ROOT}/agents/scripts/validate_ticket.py" --ticket "${TICKET_PATH}"
python3 "${ROOT}/agents/scripts/check_wip.py" --backlog "${ROOT}/agents/backlog"
python3 "${ROOT}/agents/scripts/check_ticket_scope.py" --ticket "${TICKET_PATH}" --base "${BASE_REF}"

(
  cd "${ROOT}"
  if ! command -v cargo-safe >/dev/null 2>&1; then
    echo "ERROR: cargo-safe is not available. Install/configure ~/.local/bin/cargo-safe." >&2
    exit 2
  fi
  cargo-safe check
  cargo-safe test
  cargo-safe fmt -- --check
)

python3 "${ROOT}/agents/scripts/check_qa_signoff.py" --ticket-id "${TICKET_ID}" --qa-file "${QA_PATH}"

echo "Merge gate passed for ${TICKET_ID}."
