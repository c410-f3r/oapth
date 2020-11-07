#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='postgres://oapth:oapth@localhost:5432/oapth'

test_package_with_feature "oapth" "_integration_tests,with-sqlx-postgres,with-sqlx-runtime-tokio"
test_package_with_feature "oapth" "_integration_tests,with-tokio-postgres"
