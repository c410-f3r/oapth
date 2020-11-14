#!/usr/bin/env bash

set -euxo pipefail

export RUST_BACKTRACE=1
export RUSTFLAGS='
    -D bad_style
    -D future_incompatible
    -D missing_debug_implementations
    -D missing_docs
    -D nonstandard_style
    -D rust_2018_compatibility
    -D rust_2018_idioms
    -D trivial_casts
    -D unused_qualifications
    -D warnings
'

test_package_with_feature() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Testing ${package} with features '${features}' *****\e[0m\n"
    cargo test --manifest-path "${package}"/Cargo.toml --features "${features}" --no-default-features

    tools $package "--features ${features}"
}

tools() {
    local package=$1
    local features=$2

    /bin/echo -e "\e[0;33m***** Running Clippy on ${package} | ${features} *****\e[0m\n"
    cargo clippy $features --manifest-path "${package}"/Cargo.toml -- \
        -D clippy::restriction \
        -D warnings \
        -A clippy::implicit_return \
        -A clippy::missing_docs_in_private_items

    /bin/echo -e "\e[0;33m***** Running Rustfmt on ${package} *****\e[0m\n"
    cargo fmt --all --manifest-path "${package}"/Cargo.toml -- --check
}
