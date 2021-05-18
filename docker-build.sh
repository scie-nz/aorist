#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

PLATFORMS="linux/amd64,linux/arm64"

IMG_TAG=$(date +%Y%m%d)
IMG="scienz/aorist-base:${IMG_TAG}"

echo "Building $IMG for $PLATFORMS"
docker buildx build --push --platform $PLATFORMS -f Dockerfile.ci -t $IMG .
