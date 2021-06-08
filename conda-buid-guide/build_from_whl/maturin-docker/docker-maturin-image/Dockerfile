FROM konstin2/maturin
RUN yum install -y R-core
RUN yum install -y devtoolset-7 llvm-toolset-7
#RUN scl enable devtoolset-7 llvm-toolset-7 bash
RUN yum update scl-utils 
RUN source scl_source enable devtoolset-7
RUN yum install -y epel-release
RUN yum install -y clang
ENV R_INCLUDE_DIR=/usr/lib64/R/lib
RUN printenv R_INCLUDE_DIR
RUN ln -s /usr/lib64/R/lib /usr/include/R
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH="$HOME/.cargo/bin:$PATH"
RUN yum install -y python3 R-core-devel
