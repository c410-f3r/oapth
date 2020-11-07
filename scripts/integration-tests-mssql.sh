#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mssql://sa:yourStrong_Password@localhost:1433/oapth'

test_package_with_feature "oapth" "_integration_tests,with-sqlx-mssql,with-sqlx-runtime-tokio"