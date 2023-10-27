#!/usr/bin/env bash

set -euxo pipefail

./.scripts/internal-tests-all.sh
./.scripts/integration-tests-all.sh
./.scripts/fuzz.sh