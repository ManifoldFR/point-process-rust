/*!
 * Definitions for the Hawkes process log-likelihood function.
 */
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

/// Integral term of the likelihood.
fn integral_term(
    times: Array1<f64>, mu: f64,
    alpha: f64, decay: f64, tmax: f64) -> f64
{
    let diffs = tmax - times;
    let res: f64 = diffs.par_iter()
        .fold(|| 0., |acc, v| {
            acc + 1. - (-decay*v).exp()
        }).sum();
    mu*tmax + alpha*res
}

/// Log-likelihood of the given event data under the supplied parameters.
pub fn hawkes_likelihood(
    times: ArrayView1<f64>, mu: f64,
    alpha: f64, decay: f64, tmax: f64) -> f64 {
    let times = times.to_owned();

    // compute the matrix of differences
    let n = times.shape()[0];
    let times_br = times.broadcast((n,n)).unwrap().to_owned();
    let times_t = times_br.clone(); // clone data to new array
    let times_t = times_t.t(); // transpose, borrow of clone occurs here
    let diff: Array2<f64> = times_br - times_t; // owns the data

    // println!("{:?}", diff);

    let diff_iter = diff.outer_iter();
    let a_term: Vec<f64> = diff_iter.into_par_iter().map(|line| {
        let part: f64 = line.into_par_iter().fold(|| 0., |acc, &d| {
                if d > 0. {
                    acc + (-decay*d).exp()
                } else { acc }
            }).sum();
        part
    }).collect();
    let a_term = Array::from_vec(a_term);
    let res: f64 = a_term.into_par_iter()
        .fold(|| 0., |acc, v| {
            acc + (mu+alpha*decay*v).ln()
        }).sum();

    let interm = -integral_term(times, mu, alpha, decay, tmax);
    interm + res
}
