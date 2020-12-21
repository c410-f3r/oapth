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

check_package_generic() {
    local package=$1

    /bin/echo -e "\e[0;33m***** Checking $package without features *****\e[0m\n"
    cargo check --manifest-path "$(dirname "$0")/../$package/Cargo.toml" --no-default-features

    /bin/echo -e "\e[0;33m***** Checking ${package} with all features *****\e[0m\n"
    cargo check --all-features --manifest-path "$(dirname "$0")/../$package/Cargo.toml"
}

test_package_with_features() {
    local package=$1
    shift
    local all_features=("$@")

    for features in "${all_features[@]}"; do
        /bin/echo -e "\e[0;33m***** Testing $package with features '$features' *****\e[0m\n"
        cargo test --features $features --manifest-path "$(dirname "$0")/../$package/Cargo.toml" --no-default-features
    done
}
