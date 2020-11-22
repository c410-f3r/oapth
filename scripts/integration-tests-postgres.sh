#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='postgres://oapth:oapth@localhost:5432/oapth?sslmode=disable'

test_package_with_feature "oapth" "_integration-tests,dev-tools,with-diesel-postgres"
test_package_with_feature "oapth" "_integration-tests,dev-tools,with-sqlx-postgres,_sqlx_hack"
test_package_with_feature "oapth" "_integration-tests,dev-tools,with-tokio-postgres"
