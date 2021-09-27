import sys
import os
from setuptools import setup
from setuptools_rust import Binding, RustExtension

# We set a specific cargo build target in windows
if os.name == "nt":
    os.environ["CARGO_BUILD_TARGET"] = "x86_64-pc-windows-gnu"

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

