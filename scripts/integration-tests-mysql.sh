#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mysql://oapth:oapth@localhost:3307/oapth'

test_package_with_feature "oapth" "_integration_tests,dev-tools,with-mysql_async"
test_package_with_feature "oapth" "_integration_tests,dev-tools,with-sqlx-mysql,with-sqlx-runtime-tokio"