#!/bin/sh

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $SCRIPT_DIR

set -euo pipefail
IFS=$'\n\t'

COMMIT_SHA=$(git rev-parse HEAD | cut -b 1-7)
docker build -t scienz/aorist:$COMMIT_SHA .
