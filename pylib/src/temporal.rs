//! Wrappers for the `pointprocesses::temporal` module.
use pointprocesses::temporal::*;
use traits::TemporalProcess;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::thread;
use numpy::{PyArray1,ToPyArray};
use ndarray::prelude::*;



#[pymodule]
/// Time-dependent point processes.
fn temporal(_py: Python, module: &PyModule) -> PyResult<()> {
    
    #[pyfn(module, "poisson_process")]
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

    #[pyfn(module, "variable_poisson")]
    /// A variable Poisson process on the real line.
    /// 
    /// Args:
    ///     tmax (float): upper time bound.
    ///     lambda (func): intensity function object.
    ///     max_lambda (float): upper bound on the intensity.
    ///
    /// Returns:
    ///     events (Tuple[ndarray,ndarray]): events[0] are the timestamps, events[1] are the intensities
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
        let times: Array1<f64> = elements.timestamps;
        let inten: Array1<f64> = elements.intensities;
        let times = times.to_pyarray(py).to_owned();
        let inten = inten.to_pyarray(py).to_owned();
        (times, inten)
    }

    #[pyfn(module, "hawkes_exp")]
    /// A Hawkes process on the real line with an exponential kernel.
    /// 
    /// Args:
    ///     tmax (float): temporal horizon.
    ///     alpha (float): jump size
    ///     beta (float): decay parameter.
    ///     lambda0 (float): base, background intensity.
    /// Returns:
    ///     events (Tuple[ndarray,ndarray]):
    ///         events[0] are the timestamps,
    ///         events[1] are the intensities
    fn hawkes_exp_py(
        py: Python, tmax: f64,
        alpha: f64, beta: f64, lambda0: f64
        ) -> (Py<PyArray1<f64>>, Py<PyArray1<f64>>)
    {
        let events = hawkes_exponential(
            tmax, alpha, beta, lambda0);
        let timestamps = events.timestamps.to_pyarray(py);
        let intensities = events.intensities.to_pyarray(py);
        (timestamps.to_owned(), intensities.to_owned())
    }

 
    /// Simulate a batch of Hawkes-exponential trajectories.
    /// Use this instead of a Python loop.
    /// 
    /// Args:
    ///     tmax (float): temporal horizon.
    ///     alpha (float): jump size
    ///     beta (float): decay parameter.
    ///     lambda0 (float): base, background intensity.
    ///     num_samples (int): number of samples
    #[pyfn(module, "batch_hawkes_exp")]
    fn batch_hawkes_exp(
        py: Python,
        tmax: f64, 
        alpha: f64, beta: f64, lambda0: f64,
        num_samples: usize) -> &PyList
    {
        let model = hawkes::ExpHawkes::new(alpha, beta, lambda0);
        let trajs: Vec<TimeProcessResult> = model.batch_sample(tmax, num_samples);
        convert_vec_results_to_pyarray_list(py, trajs)
    }

    Ok(())
}

fn convert_vec_results_to_pyarray_list(py: Python, results: Vec<TimeProcessResult>) -> &PyList
{
    let n: usize = results.len();
    let mut res_arrays = Vec::with_capacity(n);

    for i in 0..n {
        let evts = &results[i];
        let timestamps = evts.timestamps.to_pyarray(py);
        let intensities = evts.intensities.to_pyarray(py);
        res_arrays.push(
            (timestamps.to_owned(), intensities.to_owned())
        );
    }
    PyList::new(py, res_arrays)
}
