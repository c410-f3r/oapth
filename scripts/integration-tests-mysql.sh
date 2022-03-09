#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mysql://oapth:oapth@127.0.0.1:3307/oapth'

rust-tools test-with-features oapth _integration-tests,dev-tools,diesel-mysql
rust-tools test-with-features oapth _integration-tests,dev-tools,mysql_async

export DATABASE_URL='mysql://oapth:oapth@localhost:3307/oapth'

rust-tools test-with-features oapth _integration-tests,dev-tools,sqlx-mysql,_sqlx_hack
