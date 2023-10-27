#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

export DATABASE_URL='mssql://sa:yourStrong(!)Password@127.0.0.1:1433/oapth?trustServerCertificate=true'

$rt test-with-features oapth _integration-tests,sm-dev,tiberius
