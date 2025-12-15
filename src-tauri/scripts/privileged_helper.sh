#!/bin/bash

# Privileged Helper Script for Monitor Layout
# This script should be installed with SMJobBless for elevated privileges

displayplacer_path="/opt/homebrew/bin/displayplacer"
if [ ! -f "$displayplacer_path" ]; then
    displayplacer_path="/usr/local/bin/displayplacer"
fi

if [ ! -f "$displayplacer_path" ]; then
    echo "Error: displayplacer not found" >&2
    exit 1
fi

# Execute displayplacer with all arguments passed from the app
"$displayplacer_path" "$@"</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/src-tauri/scripts/privileged_helper.sh