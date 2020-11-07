#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

test_package_with_feature "oapth" "default"
test_package_with_feature "oapth" "with-mysql_async"
test_package_with_feature "oapth" "with-rusqlite"
test_package_with_feature "oapth" "with-sqlx-mssql,with-sqlx-runtime-tokio"
test_package_with_feature "oapth" "with-sqlx-mysql,with-sqlx-runtime-tokio"
test_package_with_feature "oapth" "with-sqlx-postgres,with-sqlx-runtime-tokio"
test_package_with_feature "oapth" "with-sqlx-runtime-actix"
test_package_with_feature "oapth" "with-sqlx-runtime-async-std"
test_package_with_feature "oapth" "with-sqlx-runtime-tokio"
test_package_with_feature "oapth" "with-sqlx-sqlite,with-sqlx-runtime-tokio"
test_package_with_feature "oapth" "with-tokio-postgres"

test_package_with_feature "oapth-cli" "default"