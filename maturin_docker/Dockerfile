FROM konstin2/maturin 
WORKDIR /
ADD https://github.com/Kitware/CMake/releases/download/v3.19.1/cmake-3.19.1.tar.gz .
RUN tar -xf cmake-3.19.1.tar.gz && \
    cd cmake-3.19.1 && \
    ./bootstrap -- -DCMAKE_USE_OPENSSL=OFF && \
    make -j8 install && \
    cd .. && \
    rm -r cmake*
RUN yum install -y R-core
ENV R_INCLUDE_DIR=/usr/lib64/R/lib
RUN printenv R_INCLUDE_DIR
RUN ln -s /usr/lib64/R/lib /usr/include/R

RUN yum install -y devtoolset-7 llvm-toolset-7
#RUN scl enable devtoolset-7 llvm-toolset-7 bash
RUN yum install -y epel-release
RUN yum install -y clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH="$HOME/.cargo/bin:$PATH"
RUN yum install -y python3 R-core-devel
WORKDIR /io
ENTRYPOINT ["/usr/bin/maturin"]




#RUN yum install -y R-core
#RUN yum install -y devtoolset-7 llvm-toolset-7
#RUN scl enable devtoolset-7 llvm-toolset-7 bash
#RUN yum install -y epel-release
#RUN yum install -y clang
#ENV R_INCLUDE_DIR=/usr/lib64/R/lib
#RUN printenv R_INCLUDE_DIR
#RUN ln -s /usr/lib64/R/lib /usr/include/R
#RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
#ENV PATH="$HOME/.cargo/bin:$PATH"
#RUN yum install -y python3 R-core-devel
