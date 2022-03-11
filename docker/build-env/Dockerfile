# Builds an image containing prerequisites for building aorist in CI.
#
# How to build this:
# 1. Get latest version of docker, ensure it's started with 'systemctl start docker'
# 2. Install qemu binfmt support: ubuntu:qemu-system-misc, arch:(aur)qemu-user-static-bin
# 3. Run: ./build.sh

FROM debian:bullseye

# Install curl for use below
RUN apt-get update -y \
  && apt-get install -y curl \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives

# DIY the rust install since it's pretty fast and it allows picking specific rust versions.
# Edited from https://github.com/rust-lang/docker-rust/blob/master/Dockerfile-slim.template
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.59.0
RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
        i386) rustArch='i686-unknown-linux-gnu' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    curl -o rustup-init "https://static.rust-lang.org/rustup/archive/1.24.3/${rustArch}/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Install other aorist build prereqs:
# - git: for pulling the repo itself in a self-contained environment
# - libclang-dev, r-base, r-base-dev: for R code generation (libclang for bindgen)
# - python3-*, black: for Python linking and code generation
# - libssl-dev: for sccache below which wants openssl-sys
RUN apt-get update -y \
  && apt-get install -y \
    git \
    libclang-dev r-base r-base-dev \
    python3-dev python3-astor python3-yaml black \
    libssl-dev \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives

# Install sccache for easy build artifact caching in CI
RUN cargo install sccache \
  && rm -rf ~/.cargo
