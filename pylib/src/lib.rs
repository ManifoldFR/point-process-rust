#![feature(extern_prelude, use_extern_macros, specialization)]

extern crate pointprocesses;
extern crate pyo3;
extern crate numpy;

use pointprocesses::*;

use pyo3::prelude::*;
use numpy::{PyArray, PyArrayModule};

/// The point process library.
#[pymodinit]
fn pointprocesses(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "poisson_process")]
    fn poisson_process_py(py: Python, tmax: f64, lambda: f64) -> PyArray<f64> {
        let np = PyArrayModule::import(py).unwrap();
        let arr = poisson_process(tmax, lambda);
        let result = PyArray::from_ndarray(py, &np, arr);
        result
    }
    
    // TODO
    /*
    #[pyfn(m, "variable_poisson")]
    fn variable_poisson_py<F>(
        _py: Python, tmax: f64, lambda: F,
        max_lambda: f64) -> Vec<EventWrapper>
    where F: Fn(f64) -> f64
    {
        let elements = variable_poisson(tmax, lambda, max_lambda);
        elements.into_iter().map(|ev| EventWrapper(ev)).collect()
    }
    */

    Ok(())
}