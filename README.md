# Point processes in Rust

[![Crates.io Status](https://img.shields.io/crates/v/point_process.svg)](https://crates.io/crates/point_process)
[![Docs](https://docs.rs/point_process/badge.svg)](https://docs.rs/point_process)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/ManifoldFR/point-process-rust/master/LICENSE)

Point processes are stochastic processes with a wide range of applications in seismology, epidemiology, or financial mathematics. They are utilized to model the arrival of random events as a function of time.

![variablepoisson](lib/examples/images/oscillating_poisson.svg)

This crate provides functions to simulate point processes in [Rust](https://rust-lang.org), built on top of [`ndarray`](https://github.com/bluss/ndarray). There is a Rust API available through the base crate as well as a Python library crate.

## Overview

### Time-dependent processes

The following time-dependent point processes have been implemented within the `timedependent` module:

* Poisson point process (homogeneous and inhomogeneous, with custom function)
* Exponential-kernel Hawkes processes, using a linear-time simulation algorithm.

### n-dimensional processes

![2dpoisson_circle](lib/examples/images/2d_poisson.variable.circle.svg)

The `generalized` module provides functions for higher-dimensional processes.

For now, only Poisson processes have been implemented.


## Python package

An Python wrapper crate is available in the [`pylib`](./pylib) directory.


## Examples

Some examples require a yet unpublished version of milliams' [plotlib](https://github.com/milliams/plotlib) graphing library. To build them, you'll need to checkout plotlib locally:

```bash
git clone https://github.com/milliams/plotlib
```

To run the examples, do for instance

```bash
cd lib/
cargo run --example variable_poisson
```

Some will produce SVG image files in the [`lib/examples`](./lib/examples) directory.

The examples show how to use the API.

## Building locally

To compile the Rust library, do

```bash
cd lib/
cargo build
```

To build the Python library,

```bash
cd pylib/
cargo build --release
```
**Warning** on macOS, you might need to add the following to `~/.cargo/config` (see [PyO3's README](https://github.com/PyO3/pyo3)):
```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```
or linking with the C compiler will fail.

To compile both crates at the same time, just do
```
cargo build
```
