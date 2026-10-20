#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
