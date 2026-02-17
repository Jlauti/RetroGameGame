#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT}"

if ! git rev-parse --git-dir >/dev/null 2>&1; then
  echo "ERROR: not in a git repository: ${ROOT}" >&2
  exit 2
fi

git fetch origin --prune

if git show-ref --verify --quiet refs/remotes/origin/main; then
  git branch -f main origin/main
else
  echo "ERROR: origin/main not found" >&2
  exit 2
fi

if git show-ref --verify --quiet refs/remotes/origin/develop; then
  git branch -f develop origin/develop
else
  # Bootstrap local+remote develop from main if it does not exist yet.
  git branch -f develop main
  git push -u origin develop
fi

git branch --set-upstream-to=origin/main main >/dev/null 2>&1 || true
git branch --set-upstream-to=origin/develop develop >/dev/null 2>&1 || true

echo "Core branch sync complete."
echo "main    -> $(git rev-parse --short main) (origin/main)"
echo "develop -> $(git rev-parse --short develop) (origin/develop)"
