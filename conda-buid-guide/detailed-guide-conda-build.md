# BUILDING AORIST LIBRARY (Detailed)
## 1. Tools required
- [conda-build](https://docs.conda.io/_/downloads/conda-build/en/latest/pdf/) 
```python
conda install conda-build # Install conda-build
conda update conda # Update conda-build
conda update conda-build # Update conda-build
```

- [setuptools](https://pypi.org/project/setuptools/) 
```python
pip install setuptools # Install setuptools
```

- [wheel](https://pypi.org/project/setuptools-rust/): used for creating wheel files
```python
pip install wheel # Install wheel
```

- [setuptools-rust](https://pypi.org/project/setuptools-rust/): used for creating Rust-based .wh file (bdist)
```python
pip install setuptools-rust # Install setuptools-rust
```
- [maturin](https://pypi.org/project/maturin/): used for creating Rust-based .wh (bdist) and tar.gz (sdist) files  
```python
pip install maturin # Install maturin
```
or

```python
conda config --add channels conda-forge # Add a new channel
conda install maturin # Install maturin
```

## 2. Creating bdist files 
### 2.1. Using setuptools-rust
#### 2.1.1. Preparing files
- `setup.py` file
```python
import sys
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="aorist",
    version="0.0.1",
    rust_extensions=[RustExtension("aorist.aorist", binding=Binding.PyO3)],
    packages=["aorist"],
    # Rust extensions are not zip safe
    zip_safe=False,
    long_description="""
    Aorist: ETL code generation for flexible environments and infrastructure
    """,
    install_requires=["astor==0.8.1", "black"],
    long_description_content_type="text/x-rst"
)
```
- `MANIFEST.in` file
```python
include Cargo.toml
recursive-include src *
recursive-include aorist_primitives *
recursive-include aorist_derive *
recursive-include aorist_concept *
recursive-include aorist_util *
recursive-include type_macro_helpers *
```
- `pyproject.toml` file
```python
[build-system]
requires = ["setuptools", "wheel", "setuptools-rust"]
```
- `build-wheels.sh` file
```python
#!/bin/bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
export PATH="$HOME/.cargo/bin:$PATH"

cd /io

for PYBIN in /opt/python/cp{36,37,38,39}*/bin; do
    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
        "${PYBIN}/python" setup.py bdist_wheel
        done

        for whl in dist/*.whl; do
            auditwheel repair "$whl" -w dist/
            done
```
#### 2.1.2. Building crates
```python
python ./setup.py develop
```
By default, `develop` will create a debug build, while `install` will create a release build.

#### 2.1.3. Pulling Docker image for manylinux
```python
docker pull quay.io/pypa/manylinux2014_x86_64 # Pull the manylinux2014 Docker image:
```

#### 2.1.4. Rebuilding Docker image for manylinux (Aorist library)
- Orignal script (not working for our lib)
```python
docker run --rm -v `pwd`:/io quay.io/pypa/manylinux2014_x86_64 /io/build-wheels.sh
```
- Creating new image named `docker-conda-image`
```Python
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
- Preparing `build.sh` file
```python
#!/bin/bash
docker build -t 'docker-conda-image'
```
- Building `docker-conda-image` 
```python
chmod +x ./build.sh
./build.sh
```
or
```python
chmod +x sudo ./build.sh # If permission is required
sudo ./build.sh # If permission is required
```
#### 2.1.5. Creating manylinux wheel file for Aorist library
```python
docker run --rm -v `pwd`:/io docker-conda-image /io/build-wheels.sh
```
or
```python
sudo docker run --rm -v `pwd`:/io docker-conda-image /io/build-wheels.sh # If permission is required
```
The created files include one `singlelinux` and one `manylinux` wheel files for each Python version (3.6, 3.7, 3.8, 3.9) and these files will be in `dist` folder

### 2.2. Using maturin
#### 2.2.1 Preparing files
- `pyproject.toml` file
```python
[build-system]
requires = ["maturin>=0.10,<0.11"]
build-backend = "maturin"
```
If a `pyproject.toml` with a `build-system` entry is present, maturin will build a source distribution (`sdist`) of your package, unless `--no-sdist` is specified

#### 2.2.2 Building crates
Run the command in containing folder containing `pyproject.toml`
```python
maturin build
```

#### 2.2.3. Rebuilding Docker image for manylinux (Aorist library)
- Orignal script (not working for our lib)
```python
docker run --rm -v $(pwd):/io konstin2/maturin build --release
```

- Creating new image named `docker-maturin-new-image`
```python
FROM docker-conda-image as builder

ENV PATH /root/.cargo/bin:$PATH

# Compile dependencies only for build caching
ADD Cargo.toml /maturin/Cargo.toml
ADD Cargo.lock /maturin/Cargo.lock
RUN mkdir /maturin/src && \
    touch  /maturin/src/lib.rs && \
    echo 'fn main() { println!("Dummy") }' > /maturin/src/main.rs && \
    cargo rustc --bin maturin --manifest-path /maturin/Cargo.toml --release -- -C link-arg=-s

ADD . /maturin/

# Manually update the timestamps as ADD keeps the local timestamps and cargo would then believe the cache is fresh
RUN touch /maturin/src/lib.rs /maturin/src/main.rs

RUN cargo rustc --bin maturin --manifest-path /maturin/Cargo.toml --release -- -C link-arg=-s \
    && mv /maturin/target/release/maturin /usr/bin/maturin \
    && rm -rf /maturin

FROM docker-conda-image

ENV PATH /root/.cargo/bin:$PATH
# Add all supported python versions
ENV PATH /opt/python/cp36-cp36m/bin/:/opt/python/cp37-cp37m/bin/:/opt/python/cp38-cp38/bin/:/opt/python/cp39-cp39/bin/:$PATH
# Otherwise `cargo new` errors
ENV USER root

RUN curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && python3 -m pip install --no-cache-dir cffi \
    && mkdir /io

COPY --from=builder /usr/bin/maturin /usr/bin/maturin

WORKDIR /io

ENTRYPOINT ["/usr/bin/maturin"]
```
- Preparing `build.sh` file
```python
#!/bin/bash
docker build -t 'docker-maturin-new-image' # If permission is required
```
- Building `docker-maturin-new-image` 
```python
chmod +x ./build.sh
./build.sh
```
or
```python
chmod +x sudo ./build.sh # If permission is required
sudo ./build.sh # If permission is required
```

#### 2.2.3. Creating manylinux wheel file for Aorist library
```python
docker run --rm -v $(pwd):/io docker-maturin-new-image build --release  # or other maturin arguments
```
The created files include one `tar.gz`  and four `manylinux` wheel files for each Python version (3.6, 3.7, 3.8, 3.9) and these files will be in `target/build/wheels` folder

