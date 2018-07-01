# Point processes in Rust

[![Crates.io Status](https://img.shields.io/crates/v/point_process.svg)](https://crates.io/crates/point_process)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/ManifoldFR/point-process-rust/master/LICENSE)

Point processes are stochastic processes with a wide range of applications in seismology, epidemiology, or financial mathematics. They are utilized to model the arrival of random events as a function of time.

![variablepoisson](coverimg.png)

This crate provides functions to simulate point processes in [Rust](https://rust-lang.org).

For now, only some one-dimensional point processes have been implemented:

* Poisson point process (homogeneous and inhomogeneous, with custom function)
* Hawkes processes (exponential kernel, see [@DassiosZhao13])

## Examples

To run the examples, do for instance

```bash
cargo run --example variable_poisson
```

It will display a plot using [gnuplot](http://www.gnuplot.info/) with SiegeLord's [RustGnuplot](https://github.com/SiegeLord/RustGnuplot).

On Windows (see [issue here](https://github.com/SiegeLord/RustGnuplot#29)), `cargo run` is broken. You can grab a plot with:

```bash
gnuplot -p < test.gnuplot
```

The examples show how to use the API.

[@DassiosZhao13]: http://eprints.lse.ac.uk/51370/1/Dassios_exact_simulation_hawkes.pdf "Exact simulation of Hawkes process with exponentially decaying intensity"