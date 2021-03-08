from setuptools import setup, find_packages

setup(
    name='rusty-slm',
    version='1.0.0',
    url='https://github.com/maxastyler/rusty-slm',
    author='Max Tyler',
    author_email='maxastyler@gmail.com',
    description='Functions to interact with an SLM',
    packages=find_packages(include=["rusty-slm"]),
    install_requires=["grpcio>=1.35", "grpcio-tools>=1.35"]
)
