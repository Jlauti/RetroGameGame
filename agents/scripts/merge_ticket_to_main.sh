#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <TICKET_ID> [BASE_REF]" >&2
  exit 2
fi

TICKET_ID="$1"
BASE_REF="${2:-main}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_BRANCH="$(cd "${ROOT}" && git rev-parse --abbrev-ref HEAD)"

if [ "${SOURCE_BRANCH}" = "${BASE_REF}" ]; then
  echo "ERROR: merge must be executed from a ticket branch, not ${BASE_REF}" >&2
  exit 1
fi

bash "${ROOT}/agents/scripts/verify_merge_gate.sh" "${TICKET_ID}" "${BASE_REF}"

(
  cd "${ROOT}"
  git switch "${BASE_REF}"
  git merge --no-ff "${SOURCE_BRANCH}" -m "merge(${TICKET_ID}): integrate ${SOURCE_BRANCH}"
)

echo "Merged ${SOURCE_BRANCH} into ${BASE_REF} for ${TICKET_ID}."
