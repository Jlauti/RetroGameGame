#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 2 ]]; then
  cat <<'USAGE'
Usage: remote-cargo.sh <ssh-target> <cargo args...>

Example:
  ./scripts/remote_compiler/remote-cargo.sh root@192.168.1.50 build --release

Environment:
  REMOTE_PORT        SSH port on the remote compiler (default: 2222)
  REMOTE_PROJECT_DIR Project path on the remote compiler (default: /work/RetroGameGame)
  SYNC_TARGET_BACK   Sync `target/` from remote to local after compile (default: 1)
USAGE
  exit 1
fi

ssh_target="$1"
shift

remote_port="${REMOTE_PORT:-2222}"
remote_project_dir="${REMOTE_PROJECT_DIR:-/work/RetroGameGame}"
sync_target_back="${SYNC_TARGET_BACK:-1}"

repo_root="$(git rev-parse --show-toplevel)"

echo "Syncing source to ${ssh_target}:${remote_project_dir} ..."
rsync -az --delete \
  --exclude=".git/" \
  --exclude="target/" \
  --exclude="target_shared/" \
  --exclude=".idea/" \
  --exclude=".vscode/" \
  --exclude="*.log" \
  -e "ssh -p ${remote_port}" \
  "${repo_root}/" \
  "${ssh_target}:${remote_project_dir}/"

echo "Running cargo remotely: cargo $*"
ssh -p "${remote_port}" "${ssh_target}" bash -s -- "${remote_project_dir}" "$@" <<'EOF'
set -euo pipefail
remote_project_dir="$1"
shift

cd "${remote_project_dir}"
cargo "$@"
EOF

if [[ "${sync_target_back}" == "1" ]]; then
  echo "Syncing remote target/ back to local target/ ..."
  rsync -az \
    -e "ssh -p ${remote_port}" \
    "${ssh_target}:${remote_project_dir}/target/" \
    "${repo_root}/target/"
fi
