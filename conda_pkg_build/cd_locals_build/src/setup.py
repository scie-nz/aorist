import setuptools
from distutils.core import setup, Extension
import distutils.command.bdist_conda

setup(
    name="eg_module",
    version="0.1",
    distclass=distutils.command.bdist_conda.CondaDistribution,
    conda_buildnum=1,
)
