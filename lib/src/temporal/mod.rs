/*!
 *Temporal point processes
 *on the real half-line $[0,\infty[$, such as Poisson or
 *Hawkes processes.
 */
pub mod traits;
pub mod poisson;
pub mod cox;
pub mod hawkes;
pub mod utils;

/// Reexport traits 
pub use traits::*;

pub use poisson::*;
pub use utils::simulate_brownian;

use ndarray::prelude::*;

/// Simulate a Poisson process with constant intensity $\lambda$.
pub fn poisson_process(tmax: f64, lambda: f64) -> Array1<f64>
{
    let process = PoissonProcess::new(lambda);
    process.sample(tmax).timestamps
}

/// Simulate a Poisson process with variable intensity.
pub fn variable_poisson<F>(tmax: f64, lambda: &F, max_lambda: f64) -> TimeProcessResult
where F: Fn(f64) -> f64 + Send + Sync
{
    let process = VariablePoissonProcess::new(lambda, max_lambda);
    let result = process.sample(tmax);
    result
}

/// Simulate a Hawkes process with an exponential kernel and constant background intensity.
pub fn hawkes_exponential(tmax: f64, alpha: f64, beta: f64, lambda0: f64) -> TimeProcessResult
{
    use hawkes::ExpHawkes;
    let model: ExpHawkes = ExpHawkes::new(alpha, beta, lambda0);
    model.sample(tmax)
}
