#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

rust-tools check-generic oapth-cli
rust-tools test-with-features oapth-cli dev-tools
rust-tools test-with-features oapth-cli mssql
rust-tools test-with-features oapth-cli mysql
rust-tools test-with-features oapth-cli pg
rust-tools test-with-features oapth-cli sqlite
