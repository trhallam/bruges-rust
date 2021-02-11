### bruges_rs setup.py

from setuptools import setup, find_packages

readme = open("README.rst").read()

install_requires = [
    "numpy",
]

testing_requires = []
extras_require = []

setup(
    name="bruges_rs",
    description="",
    long_description=readme,
    long_description_content_type="text/x-rst",
    author="...",
    install_requires=install_requires,
    packages=find_packages(),
    package_data={"": ["bruges_rs/lib/*"]},
    include_package_data=True,
    python_requires=">3.8",
)