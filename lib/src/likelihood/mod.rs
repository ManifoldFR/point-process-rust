/*!
 * Log-likelihood estimator functions.
 */
mod hawkes;

use ndarray::prelude::*;
pub use self::hawkes::hawkes_likelihood;

/**
 * The log-likelihood of a Poisson process.
 */
pub fn poisson_likelihood(data: ArrayView2<f64>, lambda: f64, tmax: f64) -> f64 {
    let n = data.shape()[1];
    -lambda*tmax + (n as f64)*lambda.ln()
}
