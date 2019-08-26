# Python package

![Yes, this is matplotlib](examples/estimate.png)

High level API for `pointprocesses` as a Python library.

This is a work in progress, much of the core library's functionality hasn't been ported yet.

## Installation

This will install the [setuptools-rust](https://github.com/PyO3/setuptools-rust) package to use the convenient PyO3 bindings inside setuptools, if you don't already have it.

```bash
python setup.py install
```

On macOS, you might need to add the following to `~/.cargo/config`, as per [PyO3's README](https://github.com/PyO3/pyo3):
```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

Check it works by importing it inside a Python terminal, outside of this directory, else Python will import the local `pointprocesses` module which isn't loaded with the library.

```python
import numpy as np
import pointprocesses as pp
```

There are working examples in the `pylib/examples/` directory.
