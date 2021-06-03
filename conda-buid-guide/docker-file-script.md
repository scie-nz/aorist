# Docker File Script
Orignal [manylinux2014_x86_64](quay.io/pypa/manylinux2014_x86_64) Docker image  doesn't 'work for the Aorist library, we rebuilt our Docker image for manylinux conversion named `docker-conda-image`. To create `docker-conda-image`, create `Dockerfile` and `build.sh` following the scripts and run the commands in the `quick-guide-conda-build.md` document.

### `Dockerfile` 
```python
FROM quay.io/pypa/manylinux2014_x86_64
RUN yum install -y R-core
RUN yum install -y devtoolset-7 llvm-toolset-7
RUN scl enable devtoolset-7 llvm-toolset-7 bash
RUN yum install -y epel-release
RUN yum install -y clang
ENV R_INCLUDE_DIR=/usr/lib64/R/lib
RUN printenv R_INCLUDE_DIR
RUN ln -s /usr/lib64/R/lib /usr/include/R
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH="$HOME/.cargo/bin:$PATH"
RUN yum install -y python3 R-core-devel
```
### `build.sh`
```python
#!/bin/bash
docker build -t 'docker-conda-image' .
```