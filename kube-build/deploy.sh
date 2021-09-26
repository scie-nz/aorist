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

# set namespace, then reset back to current afterwards
# this allows us to apply across namespaces in a single 'apply' command, while still having an assigned default
TARGET_NAMESPACE=walden
ORIG_NAMESPACE=$(kubectl config view --minify --output 'jsonpath={..namespace}')
if [ -z "$ORIG_NAMESPACE" ]; then
    ORIG_NAMESPACE=$TARGET_NAMESPACE
fi

reset_namespace() {
    echo "Switching back to namespace: $ORIG_NAMESPACE"
    kubectl config set-context --current --namespace=$ORIG_NAMESPACE
}
trap reset_namespace EXIT

echo "Switching to namespace: $TARGET_NAMESPACE"
# if namespace doesn't exist, create it
kubectl create namespace $TARGET_NAMESPACE --dry-run=client -o yaml | kubectl apply -f -
kubectl config set-context --current --namespace=$TARGET_NAMESPACE

helm template -g $SCRIPT_DIR | kubectl apply -f -
