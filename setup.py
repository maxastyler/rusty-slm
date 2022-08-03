from setuptools import setup, find_packages

setup(
    name="rusty_slm",
    version="1.0.0",
    url="https://github.com/maxastyler/rusty-slm",
    author="Max Tyler",
    author_email="maxastyler@gmail.com",
    description="Functions to interact with an SLM",
    packages=find_packages(include=["rusty_slm"]),
    install_requires=["grpcio==1.48", "grpcio-tools==1.48"],
)
