#![feature(extern_prelude, use_extern_macros, specialization)]

extern crate pointprocesses;
extern crate pyo3;
extern crate numpy;
extern crate ndarray;

use pointprocesses::*;
use std::thread;
use pyo3::prelude::*;
use numpy::*;
use ndarray::prelude::*;

/// A set of time-dependent point processes.
#[pymodinit]
fn timedependent(py: Python, m: &PyModule) -> PyResult<()> {
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // You **must** write this sentence for PyArray type checker working correctly
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    let _np = PyArrayModule::import(py)?;

    #[pyfn(m, "poisson_process")]
    /// Simulate a homogeneous, constant-intensity Poisson process.
    /// 
    /// Args:
    ///     tmax (float): upper time bound.
    ///     lambda (float): base intensity.
    /// 
    /// Returns:
    ///     Process timestamps.
    fn poisson_process_py(py: Python, tmax: f64, lambda: f64) -> PyResult<PyArray<f64>> {
        let np = PyArrayModule::import(py).unwrap();
        let arr = poisson_process(tmax, lambda);
        Ok(PyArray::from_ndarray(py, &np, arr))
    }

    #[pyfn(m, "variable_poisson")]
    /// A variable Poisson process on the real line.
    /// 
    /// Args:
    ///     tmax (float): upper time bound.
    ///     lambda (func): intensity function object.
    ///     max_lambda (float): upper bound on the intensity.
    /// Returns:
    ///     arr (ndarray): arr[:,0] are the timestamps, arr[:,1] are the intensities
    fn variable_poisson_py(
        py: Python, tmax: f64, lambda: PyObject,
        max_lambda: f64) -> PyResult<PyArray<f64>>
    {
        let ref np = PyArrayModule::import(py).unwrap();
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
        Ok(elements.into_pyarray(py, np))
    }

    Ok(())
}

/// Point processes in n-dimensional space
#[pymodinit]
fn generalized(py: Python, m: &PyModule) -> PyResult<()> {
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // You **must** write this sentence for PyArray type checker working correctly
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    let _np = PyArrayModule::import(py)?;

    #[pyfn(m, "poisson_process")]
    fn poisson_process_py(py: Python, lambda: f64, close: &PyArray<f64>, far: &PyArray<f64>) -> PyResult<PyArray<f64>> {
        assert_eq!(close.dims(), far.dims());
        let ref np = PyArrayModule::import(py).unwrap();
        let close = close.as_array().into_pyresult("close must be a f64 array")?;
        let close = close.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let far = far.as_array().into_pyresult("far must be a f64 array")?;
        let far = far.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let ref domain = generalized::Rectangle::new(close, far);
        let events = generalized::poisson_process(lambda, domain);
        Ok(events.into_pyarray(py, np))
    }

    #[pyfn(m, "variable_poisson")]
    fn variable_poisson_py(
        py: Python, lambda: PyObject,
        max_lambda: f64, close: &PyArray<f64>, far: &PyArray<f64>) -> PyResult<PyArray<f64>>
    {
        let ref np = PyArrayModule::import(py).unwrap();
        let close = close.as_array().into_pyresult("close must be a f64 array")?;
        let close = close.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let far = far.as_array().into_pyresult("far must be a f64 array")?;
        let far = far.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let domain = generalized::Rectangle::new(close, far);

        let compute = |x: PyArray<f64>| {
            let args = (x,);
            let obj: PyObject = lambda.call1(py, args).unwrap();
            let res: f64 = obj.extract::<f64>(py).unwrap();
            res
        };

        let (snder, recver) = std::sync::mpsc::sync_channel::<Array1<f64>>(1);
        let (backsnder, backrcver) = std::sync::mpsc::sync_channel::<f64>(1);
        let backrcver = std::sync::Arc::from(std::sync::Mutex::from(backrcver));
        let callback = move |x: &Array1<f64>| {
            let backrcver = backrcver.lock().unwrap();
            snder.send(x.clone()).unwrap();
            let message: Result<f64,_> = backrcver.recv();
            message.unwrap()
        };

        let handle = thread::spawn(move || {
            generalized::variable_poisson(&callback, max_lambda, &domain)
        });


        recver.iter().for_each(|x| {
            let intens = compute(x.into_pyarray(py, np));
            backsnder.send(intens).unwrap();
        });

        let elements = handle.join().unwrap();
        Ok(elements.into_pyarray(py, np))
    }

    Ok(())
}