#!/bin/bash
docker buildx build --push --platform linux/amd64,linux/arm64 -t scienz/airflow:2.0.0-$(date +%Y%m%d) .
