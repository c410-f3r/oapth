#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='file::memory:'

test_package_with_feature "oapth" "_integration_tests,dev-tools,with-diesel-sqlite"
test_package_with_feature "oapth" "_integration_tests,dev-tools,with-rusqlite"
test_package_with_feature "oapth" "_integration_tests,dev-tools,with-sqlx-sqlite,with-sqlx-runtime-async-std-rustls"