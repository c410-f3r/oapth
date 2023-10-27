#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='file::memory:'

$rt test-with-features oapth _integration-tests,sm-dev,sqlx-sqlite