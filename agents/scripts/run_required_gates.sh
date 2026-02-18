#!/usr/bin/env bash
set -euo pipefail

# Runs check/test/fmt under one compile lock to avoid gate-storm contention.
# Default behavior is fail-fast when lock is busy (exit 75).
# Use --wait to block until lock is available.

wait_mode=0
if [ "${1:-}" = "--wait" ]; then
  wait_mode=1
  shift
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LOCK_FILE="${ROOT}/.agent_compile.lock"
SHARED_TARGET_DIR="${ROOT}/target_shared"
LOG_DIR="${ROOT}/agents/status/gates"
CALLER_CWD="$(pwd -P)"
mkdir -p "${LOG_DIR}"

run_id="$(date +%Y%m%dT%H%M%S)_bundle_$$"
LOG_FILE="${LOG_DIR}/${run_id}.log"
META_FILE="${LOG_DIR}/${run_id}.meta"
start_epoch="$(date +%s)"
started_at="$(date -Is)"
check_exit="n/a"
test_exit="n/a"
fmt_exit="n/a"
interrupted=0

on_signal() {
  interrupted=1
  echo "[gate-bundle] interrupted by signal"
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
  if [ "${rc}" -eq 0 ] && { [ "${check_exit}" = "n/a" ] || [ "${test_exit}" = "n/a" ] || [ "${fmt_exit}" = "n/a" ]; }; then
    rc=86
  fi
  {
    echo "mode=bundle"
    echo "run_id=${run_id}"
    echo "script=${BASH_SOURCE[0]}"
    echo "root=${ROOT}"
    echo "caller_cwd=${CALLER_CWD}"
    echo "wait_mode=${wait_mode}"
    echo "lock_file=${LOCK_FILE}"
    echo "log_file=${LOG_FILE}"
    echo "meta_file=${META_FILE}"
    echo "start_epoch=${start_epoch}"
    echo "started_at=${started_at}"
    echo "finished_at=${finished_at}"
    echo "duration_sec=${duration_sec}"
    echo "interrupted=${interrupted}"
    echo "gate_check_exit=${check_exit}"
    echo "gate_test_exit=${test_exit}"
    echo "gate_fmt_exit=${fmt_exit}"
    echo "exit_code=${rc}"
  } > "${META_FILE}"
  echo "[gate-bundle] finished exit=${rc} duration_sec=${duration_sec}"
  echo "[gate-bundle] ledger check=${check_exit} test=${test_exit} fmt=${fmt_exit}"
}
trap cleanup EXIT

exec > >(tee -a "${LOG_FILE}") 2>&1

if ! command -v cargo-safe >/dev/null 2>&1; then
  echo "ERROR: cargo-safe is not available. Install/configure ~/.local/bin/cargo-safe." >&2
  exit 2
fi

if [ -z "${CARGO_TARGET_DIR:-}" ]; then
  export CARGO_TARGET_DIR="${SHARED_TARGET_DIR}"
fi
if [ -z "${CARGO_BUILD_JOBS:-}" ]; then
  export CARGO_BUILD_JOBS=6
fi

run_gate() {
  gate_name="$1"
  shift
  echo "[gate-bundle] >>> ${gate_name}: $*"
  if "$@"; then
    gate_rc=0
  else
    gate_rc=$?
  fi
  case "${gate_name}" in
    check) check_exit="${gate_rc}" ;;
    test) test_exit="${gate_rc}" ;;
    fmt) fmt_exit="${gate_rc}" ;;
  esac
  echo "[gate-bundle] <<< ${gate_name}: exit=${gate_rc}"
  return "${gate_rc}"
}

echo "[gate-bundle] run_id=${run_id}"
echo "[gate-bundle] caller_cwd=${CALLER_CWD}"
echo "[gate-bundle] log_file=${LOG_FILE}"
echo "[gate-bundle] meta_file=${META_FILE}"

exec 9>"${LOCK_FILE}"
if [ "${wait_mode}" -eq 1 ]; then
  echo "[gate-bundle] waiting for compile lock: ${LOCK_FILE}"
  flock 9
else
  if ! flock -n 9; then
    echo "[gate-bundle] BUSY: compile lock is held. Retry later." >&2
    exit 75
  fi
fi

echo "[gate-bundle] lock acquired"
echo "[gate-bundle] CARGO_TARGET_DIR=${CARGO_TARGET_DIR}"
echo "[gate-bundle] CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS}"

run_gate check cargo-safe check
run_gate test cargo-safe test
run_gate fmt cargo-safe fmt -- --check

if [ "${check_exit}" = "n/a" ] || [ "${test_exit}" = "n/a" ] || [ "${fmt_exit}" = "n/a" ]; then
  echo "[gate-bundle] ERROR: incomplete gate ledger (n/a present)"
  exit 86
fi

echo "[gate-bundle] completed"
