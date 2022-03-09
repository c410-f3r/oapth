#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

rust-tools test-with-features oapth rusqlite
rust-tools test-with-features oapth sqlx-mssql,_sqlx_hack
rust-tools test-with-features oapth sqlx-mysql,_sqlx_hack
rust-tools test-with-features oapth sqlx-pg,_sqlx_hack
rust-tools test-with-features oapth sqlx-sqlite,_sqlx_hack
rust-tools test-with-features oapth tiberius
rust-tools test-with-features oapth tokio-postgres
