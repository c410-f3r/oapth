#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

test_package_with_feature oapth _integration-tests
test_package_with_feature oapth default
test_package_with_feature oapth dev-tools
test_package_with_feature oapth with-diesel-mysql
test_package_with_feature oapth with-diesel-pg
test_package_with_feature oapth with-diesel-sqlite
test_package_with_feature oapth with-mysql_async
test_package_with_feature oapth with-rusqlite
test_package_with_feature oapth with-sqlx-mssql,_sqlx_hack
test_package_with_feature oapth with-sqlx-mysql,_sqlx_hack
test_package_with_feature oapth with-sqlx-pg,_sqlx_hack
test_package_with_feature oapth with-sqlx-sqlite,_sqlx_hack
test_package_with_feature oapth with-tiberius
test_package_with_feature oapth with-tokio-postgres

test_package_with_feature oapth-benchmarks default

test_package_with_feature oapth-cli default
test_package_with_feature oapth-cli dev-tools
test_package_with_feature oapth-cli mssql
test_package_with_feature oapth-cli mysql
test_package_with_feature oapth-cli pg
test_package_with_feature oapth-cli sqlite
