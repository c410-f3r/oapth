#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mysql://oapth:oapth@127.0.0.1:3306/oapth'

rust-tools test-with-features oapth _integration-tests,dev-tools,with-diesel-mysql
rust-tools test-with-features oapth _integration-tests,dev-tools,with-mysql_async
rust-tools test-with-features oapth _integration-tests,dev-tools,with-sqlx-mysql,_sqlx_hack
