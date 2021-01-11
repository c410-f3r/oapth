#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

rust-tools rustfmt
rust-tools clippy

rust-tools check-generic oapth-macros

rust-tools check-generic oapth-benchmarks

