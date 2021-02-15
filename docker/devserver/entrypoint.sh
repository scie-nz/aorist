#!/bin/bash
cd ~
export GIT_SSH_COMMAND='ssh -i /etc/deploy_key/ssh-privatekey'
ssh-add /etc/deploy_key/ssh-privatekey
mkdir -p ~/.ssh
eval "$(ssh-agent)" && ssh-keyscan github.com >> ~/.ssh/known_hosts && git clone -q git@github.com:scie-nz/aorist.git
git config --global user.email "hello@scie.nz"
git config --global user.name "Aorist Agent"
cd aorist
cargo build
while true; do sleep 30; done;

