FROM rocker/r-devel AS builder
RUN apt-get update -qq --allow-releaseinfo-change && apt-get install -y curl git clang libclang-dev python3-pip libpython3-dev
RUN pip3 install dill astor black
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh
RUN bash rust_install.sh -y
