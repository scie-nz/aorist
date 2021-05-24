import sys

from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension

setup(
    name="aorist",
    version="0.0.1",
    rust_extensions=[RustExtension("aorist.rust", path="Cargo.toml", binding=Binding.PyO3)],
    packages = ["aorist"], #find_namespace_packages(include=['aorist.*']),
    # Rust extensions are not zip safe
    zip_safe=False,
    long_description="""
    Aorist: ETL code generation for flexible environments and infrastructure
    """,
    install_requires=["astor==0.8.1", "black"],
    long_description_content_type="text/x-rst"
)
