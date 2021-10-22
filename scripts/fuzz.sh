#!/usr/bin/env bash

cargo fuzz run --fuzz-dir oapth-fuzz parsers -- -max_len=10 -runs=100000