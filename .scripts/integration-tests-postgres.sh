#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='postgres://oapth:oapth@localhost:5432/oapth?sslmode=disable'

$rt test-with-features oapth _integration-tests,sm-dev,sqlx-postgres