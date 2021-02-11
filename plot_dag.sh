#!/bin/bash
cat  | sort | uniq > out.txt && Rscript plot_dag.R
