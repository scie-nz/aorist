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
