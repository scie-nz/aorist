# Building File Script

### `setup.py` 
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

### `MANIFEST.in`
```python
include Cargo.toml
recursive-include src *
recursive-include aorist_primitives *
recursive-include aorist_derive *
recursive-include aorist_concept *
recursive-include aorist_util *
recursive-include type_macro_helpers *
```

### `pyproject.toml`
```python
[build-system]
requires = ["setuptools", "wheel", "setuptools-rust"]
```

### `build-wheels.sh`
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

### `meta.yaml`
```python
package:
  name: "aorist"
  version: "0.0.1"

build:
  number: 1
  entry_points:
    - aorist=aorist:main

requirements:
  host:
    - pip
    - python
    - setuptools-rust
    - setuptools
    - wheel
  run:
    - python
    - pip
    - wheel

about:
  home: "https://github.com/scie-nz/aorist"
  license: MIT
  license_family: MIT
  license_file: 
  summary: "Aorist is a code-generation tool for MLOps. Its aim is to generate legible code for common repetitive tasks in data science, such as data replication, common transformations, as well as machine learning operations."
  doc_url: 
  dev_url: 
```

### `build.sh`
```python
#!/bin/bash
pip install ~/aorist-test/dist/aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl 
```
If you are using different version of Python, change `aorist-0.0.1-cp38-cp38-manylinux_2_17_x86_64.manylinux2014_x86_64.whl` into another wheel file which is compatible with your Python. 