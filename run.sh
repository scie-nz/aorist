#!/bin/bash
python gen_yaml.py > snap.yaml
cp target/debug/liblib.so example/aorist.so
python example/gen_airflow.py > example.py
