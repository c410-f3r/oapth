#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

test_package_with_feature "oapth" "default"
test_package_with_feature "oapth" "with-diesel-mysql"
test_package_with_feature "oapth" "with-diesel-postgres"
test_package_with_feature "oapth" "with-diesel-sqlite"
test_package_with_feature "oapth" "with-mysql_async"
test_package_with_feature "oapth" "with-rusqlite"
test_package_with_feature "oapth" "with-sqlx-mssql,with-sqlx-runtime-tokio-rustls"
test_package_with_feature "oapth" "with-sqlx-mysql,with-sqlx-runtime-tokio-rustls"
test_package_with_feature "oapth" "with-sqlx-postgres,with-sqlx-runtime-tokio-rustls"
test_package_with_feature "oapth" "with-sqlx-sqlite,with-sqlx-runtime-tokio-rustls"
test_package_with_feature "oapth" "with-tiberius"
test_package_with_feature "oapth" "with-tokio-postgres"

test_package_with_feature "oapth-cli" "default"