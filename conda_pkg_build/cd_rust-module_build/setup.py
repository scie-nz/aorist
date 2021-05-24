from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension


setup(
    name     ='scie_rust_eg',
    version  ="0.3.0",
    packages =find_namespace_packages(include=['scie_rust_eg.*']),
    zip_safe =False,
    rust_extensions=[RustExtension("scie_rust_eg.rust", path="Cargo.toml", binding=Binding.PyO3, debug=False)],
)
