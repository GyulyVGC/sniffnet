#!/usr/bin/env zsh
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"
SNIFFNET_PATH="$SCRIPT_DIR/sniffnet"
BPF_HELPER_PATH="$SCRIPT_DIR/configure_bpf.sh"
USER_ID="$(id -u)"

osascript - "$BPF_HELPER_PATH" "$USER_ID" "$$" <<'APPLESCRIPT'
on run argv
    set helperPath to quoted form of item 1 of argv
    set userId to quoted form of item 2 of argv
    set appPid to quoted form of item 3 of argv
    do shell script helperPath & " " & userId & " " & appPid with prompt "Comfortably monitor your Internet traffic." with administrator privileges
end run
APPLESCRIPT

exec "$SNIFFNET_PATH"
