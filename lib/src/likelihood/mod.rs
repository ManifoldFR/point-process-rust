//! Utility functions to compute the log-likelihood of the data under the models.
//! The general form is given by
//! $$
//!     \ell(\Theta) = \sum_i \log(\lambda_{t_i}) - \int_0^T \lambda_t dt
//! $$
mod hawkes;

pub use hawkes::{hawkes_likelihood,HawkesLikelihood};

use ndarray::prelude::*;

use crate::temporal::{PoissonProcess, DeterministicIntensity};

/// Log-likelihood of the data under the given Poisson model
/// $$ \ell(\lambda) =
///    N\ln\lambda - \lambda T
/// $$
pub fn poisson_likelihood(
    times: ArrayView1<f64>,
    model: &PoissonProcess,
    tmax: f64) -> f64
{
    let n_events = times.len();
    let lbda = model.intensity(0.);
    n_events as f64 * lbda.ln() - lbda * tmax
}

