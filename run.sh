#!/bin/bash
cp target/debug/libaorist.so example/aorist.so
python example/gen_airflow.py > example.py
