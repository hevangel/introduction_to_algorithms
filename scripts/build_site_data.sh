#!/usr/bin/env bash
# Runs the whole test-suite, then regenerates site-data/manifest.json and logs/*.log
# (demo output + per-algorithm unit-test results) for the web app.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target
cargo test --release --lib -- --test-threads=1 2>&1 | tee target/test-output.txt | tail -n 5
cargo run --release --quiet --bin gen_site_data -- target/test-output.txt
