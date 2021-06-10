# Building File Script
To build the Conda packge, `setup.py`, `meta.yaml` and `build.sh` must be scripted as below: 

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
    long_description_content_type="text/x-rst"
)
```


### `meta.yaml`
```python
package:
  name: "aorist"
  version: "0.0.1"

source:
  path: .

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
    - conda-build
    - astor==0.8.1
    - black
  run:
    - python
    - pip
    - wheel
    - astor==0.8.1
    - black

about:
  home: "https://github.com/giswqs/whitebox"
  license: MIT
  license_family: MIT
  license_file:
  summary: "An advanced geospatial data analysis platform"
  doc_url:
  dev_url:
```

### `build.sh`
```python
#!/bin/bash
$PYTHON setup.py install
```