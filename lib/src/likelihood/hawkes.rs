/*!
 * Definitions for the Hawkes process log-likelihood function.
 */
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

use rayon::prelude::*;

use crate::temporal::hawkes::ExpHawkes;
use crate::temporal::DeterministicIntensity;

/// Integral term of the likelihood -- actually easy to compute.
fn integral_term(
    times: ArrayView1<f64>,
    lbda0: f64,
    alpha: f64,
    beta: f64,
    tmax: f64) -> f64
{
    let times = times.into_owned();
    
    // Sum the int_0^T g(t-t_i) dt
    let res: f64 = times
        .par_iter()
        .fold(|| 0., |acc, t| {
            acc + alpha / beta * (1. - (-beta*(tmax - t)).exp())
        }).sum();
    lbda0*tmax + res
}

/// Compute auto-intensities at event times
fn compute_log_intensities(
    times: ArrayView1<f64>,
    lbda0: f64,
    alpha: f64,
    beta: f64) -> f64
{
    let n_events = times.shape()[0];
    let mut r_arr = vec![0.; n_events];
    let mut decay;

    r_arr[0] = 0.;
    for i in 1..n_events {
        decay = (-beta*(times[i] - times[i-1])).exp();
        r_arr[i] = decay * r_arr[i-1] + decay;
    }

    r_arr.par_iter()
        .fold(|| 0., |acc, rk| {
            acc + (lbda0 + alpha * rk).ln()
        }).sum()
}

/// Log-likelihood of the given event data under the supplied model.
pub fn hawkes_likelihood(
    times: ArrayView1<f64>,
    model: &ExpHawkes,
    tmax: f64) -> f64
{
    let lbda0 = model.get_background().intensity(0.);
    let kernel = model.get_kernel();
    let alpha = kernel.alpha;
    let beta = kernel.beta;

    
    // Get sum of log(lambda(t_i))
    let evt_likelihood = compute_log_intensities(
        times, lbda0, alpha, beta);

    let intgrl = integral_term(
        times, lbda0, alpha, beta, tmax);
    evt_likelihood - intgrl
}
