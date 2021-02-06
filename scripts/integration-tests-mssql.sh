#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mssql://sa:yourStrong_Password@127.0.0.1:1433/oapth?trustServerCertificate=true'

rust-tools test-with-features oapth _integration-tests,dev-tools,with-sqlx-mssql,_sqlx_hack
rust-tools test-with-features oapth _integration-tests,dev-tools,with-tiberius
