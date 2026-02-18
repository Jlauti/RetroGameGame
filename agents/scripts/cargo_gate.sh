#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <cargo-safe args...>" >&2
  echo "Example: $0 check" >&2
  exit 2
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LOCK_FILE="${ROOT}/.agent_compile.lock"
SHARED_TARGET_DIR="${ROOT}/target_shared"
LOG_DIR="${ROOT}/agents/status/gates"
CALLER_CWD="$(pwd -P)"
mkdir -p "${LOG_DIR}"

if ! command -v cargo-safe >/dev/null 2>&1; then
  echo "ERROR: cargo-safe is not available. Install/configure ~/.local/bin/cargo-safe." >&2
  exit 2
fi

cmd=(cargo-safe "$@")
cmd_label="$(printf '%s' "$1" | tr -cs '[:alnum:]_.-' '_')"
started_at="$(date -Is)"
start_epoch="$(date +%s)"
run_id="${GATE_RUN_ID:-$(date +%Y%m%dT%H%M%S)_single_${cmd_label}_$$}"
LOG_FILE="${LOG_DIR}/${run_id}.log"
META_FILE="${LOG_DIR}/${run_id}.meta"
cargo_args_display="$(printf '%q ' "$@")"
interrupted=0

on_signal() {
  interrupted=1
  echo "[cargo-gate] interrupted by signal"
  exit 130
}
trap on_signal INT TERM HUP

cleanup() {
  local rc=$?
  end_epoch="$(date +%s)"
  finished_at="$(date -Is)"
  duration_sec=$((end_epoch - start_epoch))
  if [ "${interrupted}" -eq 1 ] && [ "${rc}" -eq 0 ]; then
    rc=130
  fi
  {
    echo "mode=single"
    echo "run_id=${run_id}"
    echo "script=${BASH_SOURCE[0]}"
    echo "root=${ROOT}"
    echo "caller_cwd=${CALLER_CWD}"
    echo "lock_file=${LOCK_FILE}"
    echo "cargo_args=${cargo_args_display}"
    echo "log_file=${LOG_FILE}"
    echo "meta_file=${META_FILE}"
    echo "start_epoch=${start_epoch}"
    echo "started_at=${started_at}"
    echo "finished_at=${finished_at}"
    echo "duration_sec=${duration_sec}"
    echo "interrupted=${interrupted}"
    echo "exit_code=${rc}"
  } > "${META_FILE}"
  echo "[cargo-gate] finished exit=${rc} duration_sec=${duration_sec}"
}
trap cleanup EXIT

exec > >(tee -a "${LOG_FILE}") 2>&1

if [ -z "${CARGO_TARGET_DIR:-}" ]; then
  export CARGO_TARGET_DIR="${SHARED_TARGET_DIR}"
fi
if [ -z "${CARGO_BUILD_JOBS:-}" ]; then
  export CARGO_BUILD_JOBS=6
fi

echo "[cargo-gate] run_id=${run_id}"
echo "[cargo-gate] caller_cwd=${CALLER_CWD}"
echo "[cargo-gate] log_file=${LOG_FILE}"
echo "[cargo-gate] meta_file=${META_FILE}"
echo "[cargo-gate] command=${cmd[*]}"
echo "[cargo-gate] waiting for compile lock: ${LOCK_FILE}"
echo "[cargo-gate] CARGO_TARGET_DIR=${CARGO_TARGET_DIR}"
echo "[cargo-gate] CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS}"
# Serialize heavy cargo operations so multiple agents do not thrash the same host.
flock "${LOCK_FILE}" "${cmd[@]}"
