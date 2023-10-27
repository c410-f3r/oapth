#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mysql://oapth:oapth@127.0.0.1:3306/oapth'

$rt test-with-features oapth _integration-tests,sm-dev,sqlx-mysql
