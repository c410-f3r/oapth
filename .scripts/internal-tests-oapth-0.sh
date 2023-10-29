#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

$rt test-with-features oapth ""
$rt test-with-features oapth chrono
$rt test-with-features oapth clap
$rt test-with-features oapth dotenv
$rt test-with-features oapth orm
$rt test-with-features oapth quote
$rt test-with-features oapth rust_decimal
$rt test-with-features oapth sm
$rt test-with-features oapth sm-cli
$rt test-with-features oapth sm-dev
$rt test-with-features oapth sm-dev
$rt test-with-features oapth sqlx-core
$rt test-with-features oapth sqlx-mysql
$rt test-with-features oapth sqlx-postgres
$rt test-with-features oapth sqlx-sqlite
$rt test-with-features oapth std
$rt test-with-features oapth tiberius
$rt test-with-features oapth tokio