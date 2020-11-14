#!/usr/bin/env bash

$(dirname "$0")/integration-tests-mariadb.sh &&
$(dirname "$0")/integration-tests-mssql.sh &&
$(dirname "$0")/integration-tests-mysql.sh &&
$(dirname "$0")/integration-tests-postgres.sh &&
$(dirname "$0")/integration-tests-sqlite.sh