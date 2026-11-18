#!/bin/sh

set -eu

binary=$1
log_file=$(mktemp)
trap 'rm -f "$log_file"' EXIT

set +e
timeout --kill-after=5 15 "$binary" >"$log_file" 2>&1
status=$?
set -e

if [ "$status" -eq 124 ]; then
    exit 0
fi

cat "$log_file"
echo "Sniffnet exited during startup with status $status" >&2
exit 1
