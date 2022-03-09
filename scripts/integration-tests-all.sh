#!/usr/bin/env bash

. "$(dirname "$0")/common.sh" --source-only

$(dirname "$0")/integration-tests-mariadb.sh &&
#$(dirname "$0")/integration-tests-mssql.sh &&
$(dirname "$0")/integration-tests-mysql.sh &&
$(dirname "$0")/integration-tests-pg.sh &&
$(dirname "$0")/integration-tests-sqlite.sh