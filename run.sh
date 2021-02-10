#!/bin/bash
cp target/debug/libaorist.so example/aorist.so
python3 example/gen_airflow.py > example.py
