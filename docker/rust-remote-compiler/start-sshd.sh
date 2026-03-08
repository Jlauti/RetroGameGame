#!/usr/bin/env bash
set -euo pipefail

mkdir -p /var/run/sshd /root/.ssh /work
chmod 700 /root/.ssh

if [[ ! -f /root/.ssh/authorized_keys ]]; then
  echo "Missing /root/.ssh/authorized_keys. Mount your public key file to continue."
  exit 1
fi

if [[ -w /root/.ssh/authorized_keys ]]; then
  chmod 600 /root/.ssh/authorized_keys
fi

exec /usr/sbin/sshd -D -e
