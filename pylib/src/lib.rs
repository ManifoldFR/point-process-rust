use pointprocesses::*;
use pyo3::prelude::*;
use numpy::{PyArray1};

mod temporal;
mod spatial;



/// Functions for computing the log-likelihood of events under
/// given parameters.
#[pymodule]
fn likelihood(_py: Python, module: &PyModule) -> PyResult<()> {
    
    #[pyfn(module, "hawkes_likelihood")]
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
        _py: Python,
        event_times: &PyArray1<f64>,
        lbda0: f64,
        alpha: f64, beta: f64,
        tmax: f64) -> f64 
    {
        let times = event_times.as_array();
        use pointprocesses::hawkes::ExpHawkes;
        let model = ExpHawkes::new(alpha, beta, lbda0);
        let res = likelihood::hawkes_likelihood(
            times, &model, tmax);
        res
    }

    Ok(())
}