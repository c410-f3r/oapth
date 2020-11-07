#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='file::memory:'

test_package_with_feature "oapth" "_integration_tests,with-rusqlite"
test_package_with_feature "oapth" "_integration_tests,with-sqlx-sqlite,with-sqlx-runtime-async-std"