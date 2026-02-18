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

python3 "${ROOT}/agents/scripts/validate_ticket.py" --ticket "${TICKET_PATH}"
python3 "${ROOT}/agents/scripts/check_wip.py" --backlog "${ROOT}/agents/backlog"

slug="$(echo "${TICKET_ID}" | tr '[:upper:]' '[:lower:]' | tr '_' '-')"
BRANCH="codex/${slug}"

(
  cd "${ROOT}"
  git show-ref --verify --quiet "refs/heads/${BRANCH}" && {
    echo "Branch already exists: ${BRANCH}"
    git switch "${BRANCH}"
    exit 0
  }

  git switch "${BASE_REF}"
  git switch -c "${BRANCH}"
)

echo "Created and switched to ${BRANCH}."
