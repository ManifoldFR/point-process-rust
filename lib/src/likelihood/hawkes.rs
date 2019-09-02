/*!
 * Definitions for the Hawkes process log-likelihood function.
 */
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;


use crate::temporal::hawkes::ExpHawkes;
use crate::temporal::DeterministicIntensity;

pub struct HawkesExpParams {
    lbda0: f64,
    alpha: f64,
    beta: f64
}


/// Log-likelihood of the given event data under the Hawkes model.
/// $$
///     \ell =
///     \sum_{i=1}^N \log\left(
///         \lambda_0 + \sum_{j < i} \alpha e^{-\beta(t_i-t_j)}
///     \right)
///     - \lambda_0 T 
///     - \sum_{i=1}^N \frac\alpha\beta
///     \left(1 - e^{-\beta(T - t_i)}\right)
/// $$
pub struct HawkesLikelihood<'a> {
    times: ArrayView1<'a, f64>,
    params: HawkesExpParams,
    tmax: f64,
    partial_sums: Array1<f64>,
    integral: f64
}

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

/// Compute
/// $$
///     R_i = \sum_{j < i} e^{-\beta (t_i - t_j)}
/// $$
fn compute_part_sums(
    times: ArrayView1<f64>,
    beta: f64) -> Array1<f64>
{
    let n_events = times.shape()[0];
    let mut r_arr = Array1::zeros(n_events);
    let mut decay;

    r_arr[0] = 0.;
    for i in 1..n_events {
        decay = (-beta*(times[i] - times[i-1])).exp();
        r_arr[i] = decay * r_arr[i-1] + decay;
    }

    r_arr
}

/// Compute
/// $$
///     C_i = \sum_{j < i} t_j e^{-\beta (t_i - t_j)}
/// $$
fn compute_partial_deriv_sum(
    times: ArrayView1<f64>,
    beta: f64) -> Array1<f64>
{
    let n_events = times.len();
    let mut c_arr = Array1::zeros(n_events);
    let mut decay;

    for i in 1..n_events {
        decay = (-beta * times[i+1] - times[i]).exp();
        c_arr[i] = decay * c_arr[i-1] + times[i] * decay;
    }

    c_arr
}


impl<'a> HawkesLikelihood<'a> {
    pub fn new(
        times: ArrayView1<'a, f64>,
        lbda0: f64,
        alpha: f64,
        beta: f64,
        tmax: f64) -> Self
    {
        let intgrl = integral_term(
            times, lbda0, alpha, beta, tmax);
        
        // Get the partial sums sum_{j < i} exp(-beta(ti-tj))
        let partial_sums = compute_part_sums(times, beta);

        let hl_obj = HawkesLikelihood {
            times,
            params: HawkesExpParams {lbda0, alpha, beta},
            tmax,
            partial_sums,  // move the partial sums into the struct
            integral: intgrl
        };

        hl_obj
    }

    pub fn compute_likelihood(&self) -> f64
    {
        let HawkesExpParams { lbda0, alpha, beta: _ } = self.params;
        
        let evt_llhood: f64 = self.partial_sums
            .into_par_iter()
            .fold(|| 0., |acc, rk| {
                acc + (lbda0 + alpha * rk).ln()
            }).sum();

        evt_llhood - self.integral
    }
    
    pub fn grad(&self) -> Array1<f64> {
        let HawkesExpParams {lbda0, alpha, beta} = self.params;
        let tmax = self.tmax;

        // Sum pf exp(-beta(t_i-t_j))
        let ref part_sums = self.partial_sums;


        let c_arr = compute_partial_deriv_sum(
            self.times, beta);
        
        let b_arr = self.times.to_owned() * part_sums.clone() - c_arr;


        let lbda0_deriv = part_sums.iter()
            .fold(0., |acc, r| {
                acc + 1. / (lbda0 + alpha * r)
            }) - tmax;

        let alpha_deriv = part_sums.iter()
            .fold(0., |acc, r| {
                acc + r / (lbda0 + alpha * r)
            }) - self.integral / alpha;

        let integral_beta_deriv = self.times.iter()
            .fold(0., |acc, ti| {
                acc + alpha / beta
                * (tmax - ti) * (-beta * (tmax - ti)).exp()
            });

        let beta_deriv = part_sums.iter()
            .zip(b_arr.iter())
            .fold(0., |acc, (r, b)| {
                acc + (-b)/(lbda0 + alpha * r)
            })
            + 1./beta * self.integral
            - integral_beta_deriv;

        arr1(&[lbda0_deriv, alpha_deriv, beta_deriv])
    }
}



/// Log-likelihood of the given event data under the Hawkes model.
/// $$
///     \ell =
///     \sum_{i=1}^N \log\left(
///         \lambda_0 + \sum_{j < i} \alpha e^{-\beta(t_i-t_j)}
///     \right)
///     - \lambda_0 T 
///     - \sum_{i=1}^N \frac\alpha\beta
///     \left(1 - e^{-\beta(T - t_i)}\right)
/// $$
pub fn hawkes_likelihood(
    times: ArrayView1<f64>,
    model: &ExpHawkes,
    tmax: f64) -> f64
{
    let lbda0 = model.get_background().intensity(0.);
    let kernel = model.get_kernel();
    let alpha = kernel.alpha;
    let beta = kernel.beta;

    let hl_obj = HawkesLikelihood::new(
        times,
        lbda0, alpha, beta,
        tmax);

    hl_obj.compute_likelihood()
}

