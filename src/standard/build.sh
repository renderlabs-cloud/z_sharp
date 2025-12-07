#!/bin/bash
# Compile all Luau files into a single file.

set -eu

SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"

(
	cd "$SCRIPT_DIR" && 
	darklua process --config ".darklua.jsonc" -vvv "src/core.luau" "core.luau"
)
