#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

$rt rustfmt
$rt clippy

$rt check-generic oapth
$rt check-generic oapth-benchmarks
$rt check-generic oapth-macros

