#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='postgres://oapth:oapth@localhost:5432/oapth?sslmode=disable'

test_package_with_feature "oapth" "_integration_tests,dev-tools,with-diesel-postgres"
test_package_with_feature "oapth" "_integration_tests,dev-tools,with-sqlx-postgres,with-sqlx-runtime-tokio-rustls"
test_package_with_feature "oapth" "_integration_tests,dev-tools,with-tokio-postgres"
