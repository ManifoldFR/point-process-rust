# Point processes in Rust

[![Crates.io Status](https://img.shields.io/crates/v/point_process.svg)](https://crates.io/crates/point_process)
[![Docs](https://docs.rs/point_process/badge.svg)](https://docs.rs/point_process)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/ManifoldFR/point-process-rust/master/LICENSE)

Point processes are stochastic processes with a wide range of applications in seismology, epidemiology, or financial mathematics. They are utilized to model the arrival of random events as a function of time.

![variablepoisson](variable_poisson.png)

This crate provides functions to simulate point processes in [Rust](https://rust-lang.org).

## Time-dependent processes

The following time-dependent point processes have been implemented within the `timedependent` module:

* Poisson point process (homogeneous and inhomogeneous, with custom function)
* Hawkes processes (exponential kernel, see [@DassiosZhao13])

The API returns the process trajectories as a vector of a `struct` named `Events`, which has the following fields: a timestamp, the current process intensity and a vector holding any children events (for processes with this property, *coming soon*).

## Multidimensional processes

![2dpoisson_circle](2d_poisson.circle.png)

The crate provides the `generalized` module for higher-dimensional processes.

For now, only homogeneous Poisson processes have been implemented with a function

```rust
fn poisson_process(lambda: f64, domain: &T)
    where T: Set -> ndarray::Array<f64, Ix2>
```

which takes a reference to a _domain_, that is a subset of d-dimensional space implemented with the `Set` trait (see API docs), and returns a 2-dimensional array which is a set of point events in d-dimensional space falling into the domain.

## Examples

To run the examples, do for instance

```bash
cargo run --example variable_poisson
```

Some examples display a plot using [gnuplot](http://www.gnuplot.info/) with SiegeLord's [RustGnuplot](https://github.com/SiegeLord/RustGnuplot).

On Windows (see [issue here](https://github.com/SiegeLord/RustGnuplot#29)), `cargo run` is broken. You can grab a plot with:

```bash
gnuplot -p < test.gnuplot
```

The examples show how to use the API.

[@DassiosZhao13]: http://eprints.lse.ac.uk/51370/1/Dassios_exact_simulation_hawkes.pdf "Exact simulation of Hawkes process with exponentially decaying intensity"