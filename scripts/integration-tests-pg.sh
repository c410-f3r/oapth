#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='postgres://oapth:oapth@localhost:5432/oapth?sslmode=disable'

rust-tools test-with-features oapth _integration-tests,dev-tools,with-diesel-pg
rust-tools test-with-features oapth _integration-tests,dev-tools,with-sqlx-pg,_sqlx_hack
rust-tools test-with-features oapth _integration-tests,dev-tools,with-tokio-postgres
