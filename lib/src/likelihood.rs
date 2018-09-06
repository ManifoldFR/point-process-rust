/*!
 * Log-likelihood estimator functions.
 */
use ndarray::prelude::*;

/**
 * The log-likelihood of a Poisson process.
 */
pub fn poisson_likelihood(data: ArrayView2<f64>, lambda: f64, tmax: f64) -> f64 {
    let n = data.shape()[0];
    lambda.powi(n as i32)*(-lambda*tmax).exp()
}
