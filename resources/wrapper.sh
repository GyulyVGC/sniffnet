#!/bin/sh
osascript -e '
    tell app "Terminal"
        do script "sudo /*/Sniffnet.app/Contents/MacOS/sniffnet"
        activate
    end tell
'