#!/usr/bin/env bash
set -euo pipefail

# rollback.sh — restore the previously-deployed image on the remote host.
# Mirrors the tag bookkeeping done in deploy.sh.

: "${VM_HOST:?VM_HOST is required}"
: "${VM_USER:?VM_USER is required}"

SSH_PORT="${VM_SSH_PORT:-22}"
SSH_KEY="${VM_SSH_KEY:-}"
COMPOSE_DIR="/opt/portfolio"

ssh_args=(-p "$SSH_PORT" -o StrictHostKeyChecking=accept-new -o ConnectTimeout=20)
[[ -n "$SSH_KEY" ]] && ssh_args+=(-i "$SSH_KEY")

echo ">> rolling back on $VM_USER@$VM_HOST to portfolio:prev"

ssh "${ssh_args[@]}" "$VM_USER@$VM_HOST" bash -s <<'REMOTE_SCRIPT'
set -euo pipefail
cd /opt/portfolio

if ! docker image inspect portfolio:prev >/dev/null 2>&1; then
  echo "!! no portfolio:prev image available on host" >&2
  exit 1
fi

APP_IMAGE=portfolio:prev docker compose up -d --no-deps --force-recreate portfolio
docker compose restart caddy

for i in $(seq 1 30); do
  if curl -fsS -o /dev/null --max-time 5 "http://127.0.0.1/"; then
    echo ">> rollback healthy after $((i * 2))s"
    exit 0
  fi
  sleep 2
done
echo ">> rollback failed to become healthy" >&2
exit 1
REMOTE_SCRIPT
