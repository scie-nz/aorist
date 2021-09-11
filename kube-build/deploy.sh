#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# strict mode
set -euo pipefail
IFS=$'\n\t'

# print line on error
err_report() {
    echo "Error on line $1"
}
trap 'err_report $LINENO' ERR

RELEASE_NAME=$(basename $SCRIPT_DIR)

# set namespace, then reset back to current afterwards
TARGET_NAMESPACE=walden
ORIG_NAMESPACE=$(kubectl config view --minify --output 'jsonpath={..namespace}')
if [ -z "$ORIG_NAMESPACE" ]; then
    ORIG_NAMESPACE=$TARGET_NAMESPACE
fi
reset_namespace() {
    # if namespace doesn't exist, create it
    kubectl create namespace walden --dry-run=client -o yaml | kubectl apply -f -
    echo "Switching back to namespace: $ORIG_NAMESPACE"
    kubectl config set-context --current --namespace=$ORIG_NAMESPACE
}
trap reset_namespace EXIT
echo "Switching to namespace: $TARGET_NAMESPACE"
kubectl config set-context --current --namespace=$TARGET_NAMESPACE

# force upgrade to work, otherwise get 'Error: UPGRADE FAILED: "secrets" has no deployed releases'
# might be fixed with https://github.com/helm/helm/pull/7653/
kubectl delete secret sh.helm.release.v1.${RELEASE_NAME}.v1 --ignore-not-found
# test with:
#   helm template --debug $RELEASE_NAME $SCRIPT_DIR
helm upgrade --install --debug $RELEASE_NAME $SCRIPT_DIR
