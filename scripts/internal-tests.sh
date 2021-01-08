#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

rust-tools rustfmt
rust-tools clippy

rust-tools check-generic oapth-macros

rust-tools check-generic oapth
rust-tools test-with-features oapth _integration-tests
rust-tools test-with-features oapth dev-tools
rust-tools test-with-features oapth with-diesel-mysql
rust-tools test-with-features oapth with-diesel-pg
rust-tools test-with-features oapth with-diesel-sqlite
rust-tools test-with-features oapth with-mysql_async
rust-tools test-with-features oapth with-rusqlite
rust-tools test-with-features oapth with-sqlx-mssql,_sqlx_hack
rust-tools test-with-features oapth with-sqlx-mysql,_sqlx_hack
rust-tools test-with-features oapth with-sqlx-pg,_sqlx_hack
rust-tools test-with-features oapth with-sqlx-sqlite,_sqlx_hack
rust-tools test-with-features oapth with-tiberius
rust-tools test-with-features oapth with-tokio-postgres

rust-tools check-generic oapth-benchmarks

rust-tools check-generic oapth-cli
rust-tools test-with-features oapth-cli dev-tools
rust-tools test-with-features oapth-cli mssql
rust-tools test-with-features oapth-cli mysql
rust-tools test-with-features oapth-cli pg
rust-tools test-with-features oapth-cli sqlite
