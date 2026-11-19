#!/usr/bin/env zsh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"
SNIFFNET_PATH="$SCRIPT_DIR/sniffnet"
SHELL_COMMAND="$(printf "%q " "$SNIFFNET_PATH" "$@")"
APPLE_SCRIPT_COMMAND="${SHELL_COMMAND//\\/\\\\}"
APPLE_SCRIPT_COMMAND="${APPLE_SCRIPT_COMMAND//\"/\\\"}"
osascript -e "do shell script \"$APPLE_SCRIPT_COMMAND >/dev/null 2>&1 &\" with prompt \"Comfortably monitor your network traffic.\" with administrator privileges"
