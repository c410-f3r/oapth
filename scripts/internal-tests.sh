#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

/bin/echo -e "\e[0;33m***** Running Rustfmt *****\e[0m\n"
cargo fmt --all

/bin/echo -e "\e[0;33m***** Running Clippy *****\e[0m\n"
cargo clippy --all-features -- \
    -D clippy::restriction \
    -D warnings \
    -A clippy::implicit_return \
    -A clippy::missing_docs_in_private_items

check_package_generic oapth-macros

OAPTH=(
    _integration-tests
    dev-tools
    with-diesel-mysql
    with-diesel-pg
    with-diesel-sqlite
    with-mysql_async
    with-rusqlite
    with-sqlx-mssql,_sqlx_hack
    with-sqlx-mysql,_sqlx_hack
    with-sqlx-pg,_sqlx_hack
    with-sqlx-sqlite,_sqlx_hack
    with-tiberius
    with-tokio-postgres
)
test_package_with_features oapth "${OAPTH[@]}"
check_package_generic oapth

check_package_generic oapth-benchmarks

OAPTH_CLI=(
    dev-tools
    mssql
    mysql
    pg
    sqlite
)
test_package_with_features oapth-cli "${OAPTH_CLI[@]}"
check_package_generic oapth-cli