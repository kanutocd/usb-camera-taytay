#!/usr/bin/env bash
set -euo pipefail

binary="$(cargo build --quiet --bin usb-camera-taytay --message-format=json | python3 -c 'import json,sys; print(next(json.loads(line)["executable"] for line in sys.stdin if json.loads(line).get("executable")))')"
help_output="$("$binary" --help)"
version_output="$("$binary" --version)"
[[ "$help_output" == *"Usage: usb-camera-taytay"* ]]
[[ "$version_output" == "usb-camera-taytay $(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -1)" ]]

work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT
cat > "$work_dir/config.toml" <<EOF
input_dir = "$work_dir/input"
spool_dir = "$work_dir/spool"
archive_dir = "$work_dir/archive"
source_id = "acceptance-camera"
spool_quota_bytes = 1048576
EOF

[[ "$("$binary" check-config "$work_dir/config.toml")" == "configuration is valid" ]]
[[ "$("$binary" fixture "$work_dir/config.toml")" == "published fixture.jpg" ]]
[[ "$("$binary" run-filesystem "$work_dir/config.toml")" == *"published 1 stable artifact(s)"* ]]
test -f "$work_dir/spool/ledger.jsonl"
test -f "$work_dir/input/fixture.jpg"
status_output="$("$binary" status "$work_dir/config.toml")"
artifact_id="${status_output%% *}"
[[ "$status_output" == *"Published"* ]]
[[ "$("$binary" pause "$artifact_id" "$work_dir/config.toml")" == *"Paused"* ]]
[[ "$("$binary" status "$work_dir/config.toml")" == *"Paused"* ]]
[[ "$("$binary" resume "$artifact_id" "$work_dir/config.toml")" == *"Published"* ]]
[[ "$("$binary" status "$work_dir/config.toml")" == *"Published"* ]]
