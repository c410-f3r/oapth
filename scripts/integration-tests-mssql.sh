#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mssql://sa:yourStrong_Password@127.0.0.1:1433/oapth'

test_package_with_feature "oapth" "_integration_tests,dev-tools,with-sqlx-mssql,with-sqlx-runtime-tokio-rustls"
test_package_with_feature "oapth" "_integration_tests,dev-tools,with-tiberius"