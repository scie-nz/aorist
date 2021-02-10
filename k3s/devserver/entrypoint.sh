#!/bin/bash
ssh-agent /bin/bash
ssh-add /etc/deploy_key/ssh-privatekey
ssh-keyscan github.com >> ~/.ssh/known_hosts
git clone git@github.com:scie-nz/aorist.git
git config --global user.email "hello@scie.nz"
git config --global user.name "Aorist Agent"
export GIT_SSH_COMMAND='ssh -i /etc/deploy_key/ssh-privatekey'
while true; do sleep 30; done;
