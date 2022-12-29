# Builds an image containing aorist.so along with runtime R/python prerequisites

FROM library/rust:1.66-slim-bullseye AS builder

# Aorist build prereqs:
# - libclang-dev, r-base, r-base-dev: for R code generation (libclang for bindgen)
# - python3-*, black: for Python linking and code generation
RUN apt-get update -y \
  && apt-get install -y \
    libclang-dev r-base r-base-dev \
    python3-dev python3-astor python3-yaml black \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives

WORKDIR /build
COPY . .

RUN cargo build --release \
  && ldd target/release/libaorist.so

FROM library/debian:bullseye-slim

# Install R/python/etc runtime prerequisites for using aorist
RUN apt-get update -y \
  && apt-get install -y \
      ca-certificates git \
      r-base r-base-dev \
      python3 python3-astor python3-dill python3-yaml black \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists /var/cache/apt/archives

# Copy libaorist AFTER installing prereqs to reduce fetching on libaorist rebuild
COPY --from=builder /build/target/release/libaorist.so /usr/lib/aorist.so

RUN cd /usr/lib && python3 -c "import aorist; print(aorist.__file__)"
