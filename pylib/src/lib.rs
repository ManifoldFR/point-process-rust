use pointprocesses::*;
use std::thread;
use pyo3::prelude::*;
use ndarray::prelude::*;
use numpy::{PyArray1,PyArray2,ToPyArray};

#[pymodule]
/// A set of time-dependent point processes.
fn temporal(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "poisson_process")]
    /// Simulate a homogeneous, constant-intensity Poisson process.
    /// 
    /// Args:
    ///     tmax (float): upper time bound.
    ///     lambda (float): base intensity.
    /// 
    /// Returns:
    ///     Process timestamps.
    fn poisson_process_py(py: Python, tmax: f64, lambda: f64) -> Py<PyArray1<f64>> {
        let arr = poisson_process(tmax, lambda);
        arr.to_pyarray(py).to_owned()
    }

    #[pyfn(m, "variable_poisson")]
    /// A variable Poisson process on the real line.
    /// 
    /// Args:
    ///     tmax (float): upper time bound.
    ///     lambda (func): intensity function object.
    ///     max_lambda (float): upper bound on the intensity.
    ///
    /// Returns:
    ///     arr (ndarray): arr[:,0] are the timestamps, arr[:,1] are the intensities
    fn variable_poisson_py(
        py: Python, tmax: f64, lambda: PyObject,
        max_lambda: f64) -> (Py<PyArray1<f64>>,Py<PyArray1<f64>>)
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
        let times: Array1<f64> = elements.0;
        let inten: Array1<f64> = elements.1;
        let times = times.to_pyarray(py).to_owned();
        let inten = inten.to_pyarray(py).to_owned();
        (times, inten)
    }

    #[pyfn(m, "hawkes_exp")]
    /// A Hawkes process on the real line with an exponential kernel.
    /// 
    /// Args:
    ///     tmax (float): temporal horizon.
    ///     alpha (float): jump size
    ///     beta (float): decay parameter.
    ///     lambda0 (float): base, background intensity.
    /// Returns:
    ///     arr (ndarray):
    ///         arr[:,0] are the timestamps,
    ///         arr[:,1] are the intensities
    ///         arr[:,2] are the marks
    fn hawkes_exp_py(py: Python, tmax: f64, alpha: f64, beta: f64, lambda0: f64) -> Py<PyArray2<f64>>
    {
        let events = temporal::hawkes_exponential(tmax, beta, lambda0, alpha);
        events.to_pyarray(py).to_owned()
    }

    Ok(())
}

/// Point processes in n-dimensional space
#[pymodule]
fn spatial(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "poisson_process")]
    fn poisson_process_py(py: Python, lambda: f64, close: &PyArray1<f64>, far: &PyArray1<f64>) -> Py<PyArray2<f64>> {
        assert_eq!(close.dims(), far.dims());
        let close = close.as_array();
        let close = close.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let far = far.as_array();
        let far = far.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let ref domain = spatial::Domain::new(close, far);
        let events = spatial::poisson_process(lambda, domain);
        events.to_pyarray(py).to_owned()
    }

    #[pyfn(m, "variable_poisson")]
    fn variable_poisson_py(
        py: Python, lambda: PyObject,
        max_lambda: f64, close: &PyArray1<f64>, far: &PyArray1<f64>) -> Py<PyArray2<f64>>
    {
        let close = close.as_array();
        let close = close.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let far = far.as_array();
        let far = far.to_owned()
           .into_dimensionality::<ndarray::Ix1>()
           .unwrap();
        let domain = spatial::Domain::new(close, far);

        let compute = |x: &PyArray1<f64>| {
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
            spatial::variable_poisson(&callback, max_lambda, &domain)
        });


        recver.iter().for_each(|x| {
            let intens = compute(x.to_pyarray(py));
            backsnder.send(intens).unwrap();
        });

        let elements = handle.join().unwrap();
        elements.to_pyarray(py).to_owned()
    }

    Ok(())
}

/// Functions for computing the log-likelihood of some events under
/// given parameters.
#[pymodule]
fn likelihood(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "hawkes_likelihood")]
    /// Compute the log-likelihood of the given data under a Hawkes
    /// model with the supplied parameters.
    /// 
    /// Args:
    ///     times (array): data event arrival times.
    ///     mu (float): background rate.
    ///     alpha (float): jump parameter.
    ///     decay (float): decay parameter
    ///     tmax (float): temporal horizon.
    fn hawkes_likelihood(
        _py: Python, times: &PyArray1<f64>,
        mu: f64, alpha: f64, decay: f64, tmax: f64) -> f64 
    {
        let times = times.as_array();
        let res = likelihood::hawkes_likelihood(
            times, mu, alpha, decay, tmax);
        res
    }

    Ok(())
}