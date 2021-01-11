#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

rust-tools test-with-features oapth with-rusqlite
rust-tools test-with-features oapth with-sqlx-mssql,_sqlx_hack
rust-tools test-with-features oapth with-sqlx-mysql,_sqlx_hack
rust-tools test-with-features oapth with-sqlx-pg,_sqlx_hack
rust-tools test-with-features oapth with-sqlx-sqlite,_sqlx_hack
rust-tools test-with-features oapth with-tiberius
rust-tools test-with-features oapth with-tokio-postgres
