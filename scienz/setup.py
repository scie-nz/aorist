from setuptools import setup

setup(
    name="scienz",
    version="0.0.1",
    packages=["scienz"],
    zip_safe=False,
    include_package_data=True,
    package_data={"scienz": ["scienz/*"],},
    long_description="""
    Common dataset definitions for aorist package.
    """,
    long_description_content_type="text/x-rst"
)

