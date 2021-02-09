#!/bin/bash
cp target/release/libaorist.so example/aorist.so
python example/gen_airflow.py > example.py
