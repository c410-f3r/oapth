#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

rust-tools check-generic oapth
rust-tools test-with-features oapth _integration-tests
rust-tools test-with-features oapth dev-tools
rust-tools test-with-features oapth diesel-mysql
rust-tools test-with-features oapth diesel-pg
rust-tools test-with-features oapth diesel-sqlite
rust-tools test-with-features oapth mysql_async