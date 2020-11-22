#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mysql://oapth:oapth@127.0.0.1:3307/oapth'

test_package_with_feature "oapth" "_integration-tests,dev-tools,with-diesel-mysql"
test_package_with_feature "oapth" "_integration-tests,dev-tools,with-mysql_async"

export DATABASE_URL='mysql://oapth:oapth@localhost:3307/oapth'

test_package_with_feature "oapth" "_integration-tests,dev-tools,with-sqlx-mysql,_sqlx_hack"