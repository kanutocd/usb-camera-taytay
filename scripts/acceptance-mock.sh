#!/usr/bin/env bash
set -euo pipefail

binary="$(cargo build --quiet --bin usb-camera-taytay --message-format=json | python3 -c 'import json,sys; print(next(json.loads(line)["executable"] for line in sys.stdin if json.loads(line).get("executable")))')"
help_output="$("$binary" --help)"
version_output="$("$binary" --version)"
[[ "$help_output" == *"Usage: usb-camera-taytay"* ]]
[[ "$version_output" == "usb-camera-taytay $(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -1)" ]]
