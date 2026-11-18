#!/bin/bash

set -euo pipefail

image=$1
binary=$2
smoke_script=$(realpath "${BASH_SOURCE[0]%/*}/smoke-test.sh")
runtime_dir="${RUNNER_TEMP:-/tmp}/sniffnet-wayland-${RANDOM}"
x_display=:$((RANDOM % 100 + 100))
x_socket="/tmp/.X11-unix/X${x_display#:}"

mkdir -p "$runtime_dir"
chmod 700 "$runtime_dir"

cleanup() {
    if [[ -n "${xvfb_pid:-}" ]]; then kill "$xvfb_pid" 2>/dev/null || true; fi
    if [[ -n "${weston_pid:-}" ]]; then kill "$weston_pid" 2>/dev/null || true; fi
    rm -rf "$runtime_dir"
}
trap cleanup EXIT

Xvfb "$x_display" -screen 0 1280x720x24 -ac >"$runtime_dir/xvfb.log" 2>&1 &
xvfb_pid=$!

XDG_RUNTIME_DIR="$runtime_dir" weston \
    --backend=headless-backend.so \
    --socket=wayland-smoke \
    --idle-time=0 \
    >"$runtime_dir/weston.log" 2>&1 &
weston_pid=$!

for _ in {1..50}; do
    if [[ -S "$x_socket" && -S "$runtime_dir/wayland-smoke" ]]; then
        break
    fi
    sleep 0.1
done

if [[ ! -S "$x_socket" ]]; then
    cat "$runtime_dir/xvfb.log"
    echo "Xvfb did not create its display socket" >&2
    exit 1
fi

if [[ ! -S "$runtime_dir/wayland-smoke" ]]; then
    cat "$runtime_dir/weston.log"
    echo "Weston did not create its Wayland socket" >&2
    exit 1
fi

echo "Testing $image on X11"
docker run --rm --network none \
    --cap-add NET_RAW --cap-add NET_ADMIN \
    -e DISPLAY="$x_display" \
    -e ICED_BACKEND=tiny-skia \
    -e HOME=/tmp/sniffnet-home \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v "$smoke_script:/smoke-test.sh:ro" \
    --entrypoint /bin/sh \
    "$image" /smoke-test.sh "$binary"

echo "Testing $image on Wayland"
docker run --rm --network none \
    --cap-add NET_RAW --cap-add NET_ADMIN \
    -e WAYLAND_DISPLAY=wayland-smoke \
    -e XDG_RUNTIME_DIR=/tmp/sniffnet-wayland \
    -e ICED_BACKEND=tiny-skia \
    -e HOME=/tmp/sniffnet-home \
    -v "$runtime_dir:/tmp/sniffnet-wayland" \
    -v "$smoke_script:/smoke-test.sh:ro" \
    --entrypoint /bin/sh \
    "$image" /smoke-test.sh "$binary"
