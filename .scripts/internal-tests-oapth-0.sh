#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

$rt test-with-features oapth ""
$rt test-with-features oapth sm-dev
$rt test-with-features oapth tiberius