#!/usr/bin/env bash

cargo fuzz run --features libfuzzer-sys/link_libfuzzer --fuzz-dir oapth-fuzz parsers -- -max_len=10 -runs=100000