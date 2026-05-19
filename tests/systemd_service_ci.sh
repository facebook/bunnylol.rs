#!/usr/bin/env bash
#
# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.

set -euo pipefail

container_name="bunnylol-systemd-ci-${GITHUB_RUN_ID:-local}-$$"
image_name="bunnylol-systemd-ci-image:${GITHUB_RUN_ID:-local}-$$"
repo_root="$(git rev-parse --show-toplevel)"

log() {
  echo "::group::$*"
}

end_log() {
  echo "::endgroup::"
}

cleanup() {
  docker rm -f "$container_name" >/dev/null 2>&1 || true
  docker image rm -f "$image_name" >/dev/null 2>&1 || true
}
trap cleanup EXIT

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

wait_for_container_systemd() {
  echo "Waiting for systemd to become ready in ${container_name}..."
  for _ in $(seq 1 60); do
    if docker exec "$container_name" systemctl list-units >/dev/null 2>&1; then
      echo "systemd is ready."
      return
    fi
    sleep 1
  done

  echo "error: systemd did not become ready inside container" >&2
  docker logs "$container_name" >&2 || true
  exit 1
}

require_command cargo
require_command docker

cd "$repo_root"

log "Build release binary"
cargo build --release --all-features
end_log

log "Build Ubuntu systemd test image"
docker build --tag "$image_name" - <<'DOCKERFILE'
FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      curl \
      libssl3 \
      systemd \
      systemd-sysv && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

STOPSIGNAL SIGRTMIN+3

CMD ["/sbin/init"]
DOCKERFILE
end_log

log "Start privileged systemd container"
docker run \
  --detach \
  --name "$container_name" \
  --privileged \
  --cgroupns=host \
  --tmpfs /run \
  --tmpfs /run/lock \
  --volume /sys/fs/cgroup:/sys/fs/cgroup:rw \
  --volume "$repo_root:/workspace:ro" \
  "$image_name" >/dev/null
docker ps --filter "name=${container_name}"
end_log

log "Wait for systemd"
wait_for_container_systemd
end_log

log "Install, verify, restart, and uninstall bunnylol service"
docker exec "$container_name" bash -lc '
  set -euo pipefail

  echo "Installing release binary into PATH..."
  install -m 0755 /workspace/target/release/bunnylol /usr/local/bin/bunnylol
  bunnylol --version

  echo "Removing any stale service state..."
  bunnylol service uninstall >/dev/null 2>&1 || true

  echo "Installing bunnylol systemd service..."
  bunnylol service install

  echo "Checking service is active..."
  systemctl is-active --quiet bunnylol
  systemctl status bunnylol --no-pager

  echo "Waiting for /health..."
  for _ in $(seq 1 60); do
    if curl --fail --silent --show-error http://127.0.0.1:8000/health | grep -qx ok; then
      break
    fi
    sleep 1
  done

  echo "Verifying /health..."
  curl --fail --silent --show-error http://127.0.0.1:8000/health | grep -qx ok

  echo "Verifying command redirect..."
  response_headers="$(mktemp)"
  curl --silent --show-error --output /dev/null --dump-header "$response_headers" \
    "http://127.0.0.1:8000/?cmd=gh%20facebook/react"
  cat "$response_headers"
  grep -Eq "^HTTP/[0-9.]+ 30[23]" "$response_headers"
  grep -Eiq "^location: https://github.com/facebook/react" "$response_headers"

  echo "Restarting service..."
  bunnylol service restart
  systemctl is-active --quiet bunnylol
  curl --fail --silent --show-error http://127.0.0.1:8000/health | grep -qx ok

  echo "Uninstalling service..."
  bunnylol service uninstall
  ! systemctl is-active --quiet bunnylol
  test ! -e /etc/systemd/system/bunnylol.service

  echo "Systemd service test completed successfully."
'
end_log
