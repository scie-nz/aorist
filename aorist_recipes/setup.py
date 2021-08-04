from setuptools import setup

setup(
    name="aorist_recipes",
    version="0.0.1",
    packages=["aorist_recipes"],
    zip_safe=False,
    include_package_data=True,
    package_data={"aorist_recipes": ["aorist_recipes/*"],},
    long_description="""
    Recipes for aorist package.
    """,
    long_description_content_type="text/x-rst"
)

