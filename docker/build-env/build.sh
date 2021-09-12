#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

# ideally we'd just build amd64, but blocking drone from deploying on arm doesn't work.
# so for now let's just build arm64 too
PLATFORMS="linux/amd64,linux/arm64"

# Get 7-character commit SHA (note: doesn't detect dirty commits)
COMMIT_SHA=$(git rev-parse HEAD | cut -b 1-7)

IMG="scienz/aorist-build-env:${COMMIT_SHA}"

echo "Building $IMG for $PLATFORMS"
docker buildx build --push --platform $PLATFORMS -f Dockerfile -t $IMG .
