#![feature(extern_prelude, use_extern_macros, specialization)]

extern crate pointprocesses;
extern crate pyo3;
extern crate numpy;

use pointprocesses::*;
use std::thread;
use pyo3::prelude::*;
use numpy::{PyArray, PyArrayModule};

/// The main module, written in Rust.
#[pymodinit]
fn pointprocesses(_py: Python, m: &PyModule) -> PyResult<()> {

    /// Simulate a homogeneous, constant-intensity Poisson process.
    /// index 0: timestamps
    #[pyfn(m, "poisson_process")]
    fn poisson_process_py(py: Python, tmax: f64, lambda: f64) -> PyResult<PyArray<f64>> {
        let np = PyArrayModule::import(py).unwrap();
        let arr = poisson_process(tmax, lambda);
        Ok(PyArray::from_ndarray(py, &np, arr))
    }

    /// A variable Poisson process on the real line.
    #[pyfn(m, "variable_poisson")]
    fn variable_poisson_py(
        py: Python, tmax: f64, lambda: PyObject,
        max_lambda: f64) -> PyResult<PyArray<f64>>
    {
        let compute = |x: f64| {
            let args = (x,);
            let obj: PyObject = lambda.call1(py, args).unwrap();
            let res: f64 = obj.extract::<f64>(py).unwrap();
            res
        };

        let (snder, recver) = std::sync::mpsc::sync_channel::<f64>(1);
        let (backsnder, backrcver) = std::sync::mpsc::sync_channel::<f64>(1);
        let backrcver = std::sync::Arc::from(std::sync::Mutex::from(backrcver));
        let callback = move |x: f64| {
            let backrcver = backrcver.lock().unwrap();
            snder.send(x).unwrap();
            let message: Result<f64,_> = backrcver.recv();
            message.unwrap()
        };

        let handle = thread::spawn(move || {
            variable_poisson(tmax, &callback, max_lambda)
        });


        recver.iter().for_each(|x| {
            let intens = compute(x);
            backsnder.send(intens).unwrap();
        });

        let elements = handle.join().unwrap();
        let np = PyArrayModule::import(py).unwrap();
        Ok(PyArray::from_ndarray(py, &np, elements))
    }

    Ok(())
}
