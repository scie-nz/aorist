#!/bin/bash
cargo +nightly build && ./target/debug/main | sort | uniq > out.txt && Rscript plot_dag.R
