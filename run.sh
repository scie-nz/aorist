#!/bin/bash
python gen_yaml.py >> snap.yaml
cp target/debug/liblib.so mylib.so
python entrypoint.py > example.py
black example.py
