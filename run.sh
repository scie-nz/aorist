#!/bin/bash
python gen_yaml.py > snap.yaml
cp target/debug/liblib.so aorist.so
python entrypoint.py > example.py
black example.py
