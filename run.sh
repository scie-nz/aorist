#!/bin/bash
cp target/debug/liblib.so mylib.so
python entrypoint.py > example.py
black example.py
