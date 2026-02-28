#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

PORT="${BEVY_BRP_PORT:-15702}"
TIMEOUT_SECS="${TIMEOUT_SECS:-25}"
GAME_BIN="${GAME_BIN:-./target/debug/retro-game-game}"
LOG_FILE="${LOG_FILE:-/tmp/retro_game_brp_probe.log}"

if [[ ! -x "${GAME_BIN}" ]]; then
  echo "ERROR: game binary not found or not executable: ${GAME_BIN}"
  echo "Build first (recommended):"
  echo "  env SSH_ID_FILE=~/.ssh/ghost_proxmox_ed25519 ./scripts/remote_compiler/remote-cargo.sh root@10.0.0.10 build --bin retro-game-game"
  exit 1
fi

if ! command -v codex >/dev/null 2>&1; then
  echo "ERROR: codex CLI not found in PATH"
  exit 1
fi

echo "Checking Codex MCP server registration..."
if ! codex mcp list | rg -q "^bevy-brp\\s+"; then
  echo "ERROR: MCP server 'bevy-brp' is not configured."
  echo "Add it with:"
  echo "  codex mcp add bevy-brp --env PATH=\$HOME/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin -- bevy_brp_mcp --target-url http://127.0.0.1:${PORT}"
  exit 1
fi

cleanup() {
  if [[ -n "${GAME_PID:-}" ]] && kill -0 "${GAME_PID}" >/dev/null 2>&1; then
    kill "${GAME_PID}" >/dev/null 2>&1 || true
    wait "${GAME_PID}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

echo "Launching game with BRP enabled on port ${PORT}..."
BEVY_BRP_ENABLE=1 BEVY_BRP_PORT="${PORT}" timeout "${TIMEOUT_SECS}s" "${GAME_BIN}" >"${LOG_FILE}" 2>&1 &
GAME_PID=$!

for _ in $(seq 1 30); do
  if ss -ltn | rg -q ":${PORT}\\b"; then
    break
  fi
  sleep 0.25
done

if ! ss -ltn | rg -q ":${PORT}\\b"; then
  echo "ERROR: BRP port ${PORT} did not open."
  echo "--- recent log ---"
  tail -n 50 "${LOG_FILE}" || true
  exit 2
fi

echo "Probing BRP JSON-RPC endpoint..."
PROBE_RESPONSE="$(
  curl -sS -m 3 -X POST "http://127.0.0.1:${PORT}" \
    -H "content-type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"__brp_probe__"}' || true
)"

if [[ "${PROBE_RESPONSE}" != *'"jsonrpc":"2.0"'* ]]; then
  echo "ERROR: unexpected BRP response."
  echo "Response: ${PROBE_RESPONSE}"
  echo "--- recent log ---"
  tail -n 50 "${LOG_FILE}" || true
  exit 3
fi

echo "PASS: BRP endpoint is live and responding."
echo "Probe response: ${PROBE_RESPONSE}"
echo "--- recent log ---"
tail -n 20 "${LOG_FILE}" || true
