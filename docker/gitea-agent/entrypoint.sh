#!/bin/bash
cp /aorist-master-key/* ~/.ssh/
ssh-keyscan github.com >> ~/.ssh/known_hosts
git clone git@github.com:scie-nz/aorist.git
cd aorist && cargo +nightly build

while sleep 60; do
  echo "Sleeping"
done
