#!/usr/bin/env bash

set -euxo pipefail

cargo install --git https://github.com/c410-f3r/rust-tools --force

export RUST_BACKTRACE=1
export RUSTFLAGS="$(rust-tools --template you-rust rust-flags "" -Dunused_crate_dependencies,-Dunused_lifetimes)"
