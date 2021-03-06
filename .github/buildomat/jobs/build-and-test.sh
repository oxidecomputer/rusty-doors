#!/bin/bash
#:
#: name = "build-and-test"
#: variety = "basic"
#: target = "helios"
#: rust_toolchain = "nightly-2021-09-03"
#: output_rules = [ ]
#:

set -o errexit
set -o pipefail
set -o xtrace

cargo --version
rustc --version

banner build
ptime -m cargo build
ptime -m cargo build --release

cargo fmt -- --check
cargo clippy

banner test
./lib/tests/run_tests.sh
