//! Wrappers for the `pointprocesses::spatial` module.
use pointprocesses::spatial;

use std::thread;
use pyo3::prelude::*;
use ndarray::prelude::*;
use numpy::{PyArray1,PyArray2,ToPyArray};


/// Spatial point process.
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