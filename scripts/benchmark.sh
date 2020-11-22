#!/usr/bin/env bash

set -euxo pipefail

export MSSQL='mssql://sa:yourStrong_Password@127.0.0.1:1433/oapth'
export MYSQL='mysql://oapth:oapth@127.0.0.1:3306/oapth'
export POSTGRES='postgres://oapth:oapth@localhost:5432/oapth?sslmode=disable'
export SQLITE='file::memory:'

cd oapth-benchmarks;
cargo bench -- --measurement-time 10