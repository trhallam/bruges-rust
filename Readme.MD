# bruges-rs

This is an experimental Rust based implementation of the Bruges Python library.

The idea is to see whether a low-level Rust library can be used to support
geophysical applications in higher level languages like Python or Julia so we can
share a common codebase and reduce replication across languages.

Future work should focus on bench marking and compatability tests.

## Building and Installation

Requires the Rust Nightly build due to special options in `pyo3`.

The package can be built on Windows with

`./make.bat`

### Python

Python installation is via.
```
python setup.py install
```

### Julia

Julia package has not been setup yet but the Julia folder can be included in a project.

```
include("BrugesRs.jl")
```