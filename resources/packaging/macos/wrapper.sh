#!/usr/bin/env zsh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd -P)"
SNIFFNET_PATH="$SCRIPT_DIR/sniffnet"
osascript -e "do shell script \"'$SNIFFNET_PATH' >/dev/null 2>&1 &\" with prompt \"Comfortably monitor your Internet traffic.\" with administrator privileges"
