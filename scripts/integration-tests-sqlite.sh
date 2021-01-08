#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='file::memory:'

rust-tools test-with-features oapth _integration-tests,dev-tools,with-diesel-sqlite
rust-tools test-with-features oapth _integration-tests,dev-tools,with-rusqlite
#rust-tools test-with-features oapth _integration-tests,dev-tools,with-sqlx-sqlite,_sqlx_hack
