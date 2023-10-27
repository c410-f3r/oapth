#!/usr/bin/env bash

sleep 60

/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P $MSSQL_SA_PASSWORD -d master -i setup.sql