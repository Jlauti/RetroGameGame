#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

PORT="${BEVY_BRP_PORT:-15702}"
TIMEOUT_SECS="${TIMEOUT_SECS:-45}"
GAME_BIN="${GAME_BIN:-./target/debug/retro-game-game}"
MUTATE_BLOOM="${MUTATE_BLOOM:-0}"
BLOOM_INTENSITY="${BLOOM_INTENSITY:-0.30}"

if [[ ! -x "${GAME_BIN}" ]]; then
  echo "ERROR: game binary not found: ${GAME_BIN}"
  echo "Build first:"
  echo "  env SSH_ID_FILE=~/.ssh/ghost_proxmox_ed25519 ./scripts/remote_compiler/remote-cargo.sh root@10.0.0.10 build --bin retro-game-game"
  exit 1
fi

tmp_dir="$(mktemp -d)"
log_file="${tmp_dir}/game.log"
before_file="${tmp_dir}/camera_before.json"
after_file="${tmp_dir}/camera_after.json"
lights_file="${tmp_dir}/lights.json"

cleanup() {
  if [[ -n "${game_pid:-}" ]] && kill -0 "${game_pid}" >/dev/null 2>&1; then
    kill "${game_pid}" >/dev/null 2>&1 || true
    wait "${game_pid}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

post_brp_json() {
  local payload="$1"
  local outfile="$2"
  local attempts="${3:-12}"
  for _ in $(seq 1 "${attempts}"); do
    if curl -sS -m 5 -X POST "http://127.0.0.1:${PORT}" -H "content-type: application/json" -d "${payload}" > "${outfile}"; then
      return 0
    fi
    sleep 0.2
  done
  return 1
}

post_brp_discard() {
  local payload="$1"
  local attempts="${2:-12}"
  for _ in $(seq 1 "${attempts}"); do
    if curl -sS -m 5 -X POST "http://127.0.0.1:${PORT}" -H "content-type: application/json" -d "${payload}" >/dev/null; then
      return 0
    fi
    sleep 0.2
  done
  return 1
}

echo "Launching Nebula scene with BRP on ${PORT} ..."
BEVY_BRP_ENABLE=1 RETRO_DEV_BOOT=nebula BEVY_BRP_PORT="${PORT}" timeout "${TIMEOUT_SECS}s" "${GAME_BIN}" >"${log_file}" 2>&1 &
game_pid=$!

for _ in $(seq 1 60); do
  if ss -ltn | rg -q ":${PORT}\\b"; then
    break
  fi
  sleep 0.25
done

if ! ss -ltn | rg -q ":${PORT}\\b"; then
  echo "ERROR: BRP port ${PORT} did not come up."
  tail -n 80 "${log_file}" || true
  exit 2
fi

query_camera='{
  "jsonrpc":"2.0",
  "id":101,
  "method":"world.query",
  "params":{
    "data":{
      "components":[
        "bevy_camera::camera::Camera",
        "bevy_post_process::bloom::settings::Bloom",
        "bevy_pbr::fog::DistanceFog",
        "bevy_core_pipeline::tonemapping::Tonemapping",
        "bevy_camera::projection::Projection",
        "bevy_transform::components::transform::Transform"
      ],
      "option":[],
      "has":[]
    },
    "filter":{
      "with":[
        "retro_game_game::eras::era_future::nebula_bouncer::components::NebulaGameplayCamera"
      ],
      "without":[]
    },
    "strict":false
  }
}'

query_lights='{
  "jsonrpc":"2.0",
  "id":102,
  "method":"world.query",
  "params":{
    "data":{
      "components":[
        "bevy_transform::components::transform::Transform"
      ],
      "option":[
        "bevy_light::directional_light::DirectionalLight",
        "bevy_light::point_light::PointLight"
      ],
      "has":[
        "bevy_light::directional_light::DirectionalLight",
        "bevy_light::point_light::PointLight"
      ]
    },
    "filter":{
      "with":[
        "retro_game_game::eras::era_future::nebula_bouncer::components::NebulaBouncerContext"
      ],
      "without":[]
    },
    "strict":false
  }
}'

for _ in $(seq 1 80); do
  post_brp_json "${query_camera}" "${before_file}" 3 || true
  if [[ "$(jq '.result | length' "${before_file}" 2>/dev/null || echo 0)" -gt 0 ]]; then
    break
  fi
  sleep 0.25
done

if [[ "$(jq '.result | length' "${before_file}" 2>/dev/null || echo 0)" -eq 0 ]]; then
  echo "ERROR: Nebula gameplay camera did not appear before timeout."
  echo "--- game log ---"
  tail -n 80 "${log_file}" || true
  exit 3
fi

post_brp_json "${query_lights}" "${lights_file}"

camera_entity="$(jq -r '.result[0].entity // empty' "${before_file}")"
if [[ -z "${camera_entity}" ]]; then
  echo "ERROR: Nebula gameplay camera entity missing in BRP query."
  echo "--- game log ---"
  tail -n 80 "${log_file}" || true
  exit 4
fi

if [[ "${MUTATE_BLOOM}" == "1" ]]; then
  mutate_payload="$(
    jq -nc \
      --argjson entity "${camera_entity}" \
      --argjson value "${BLOOM_INTENSITY}" \
      '{
        jsonrpc:"2.0",
        id:103,
        method:"world.mutate_components",
        params:{
          entity:$entity,
          component:"bevy_post_process::bloom::settings::Bloom",
          path:"intensity",
          value:$value
        }
      }'
  )"
  post_brp_discard "${mutate_payload}"
  sleep 0.3
fi

post_brp_json "${query_camera}" "${after_file}"

echo "BRP/MCP visual probe complete."
echo "Artifacts:"
echo "  before: ${before_file}"
echo "  after:  ${after_file}"
echo "  lights: ${lights_file}"
echo
echo "Camera entity: ${camera_entity}"
echo "Bloom intensity before:"
jq '.result[0].components["bevy_post_process::bloom::settings::Bloom"].intensity' "${before_file}"
echo "Bloom intensity after:"
jq '.result[0].components["bevy_post_process::bloom::settings::Bloom"].intensity' "${after_file}"
echo
echo "Directional lights found:"
jq '[.result[] | select(.has["bevy_light::directional_light::DirectionalLight"] == true)] | length' "${lights_file}"
echo "Point lights found:"
jq '[.result[] | select(.has["bevy_light::point_light::PointLight"] == true)] | length' "${lights_file}"
