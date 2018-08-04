/*!
This module implements a set of time-dependent point processes, such as Poisson or Hawkes processes, on the real half-line [0,∞[.
*/
use rand::prelude::*;
use rand::distributions::Uniform;
use rand::distributions::Poisson;

use ndarray::stack;
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

/// Simulate a homogeneous, constant-intensity Poisson process.
/// index 0: timestamps
pub fn poisson_process(tmax: f64, lambda: f64) -> Array1<f64> {
    /// A poisson process cannot have negative intensity.
    assert!(lambda > 0.0);
    let mut rng = thread_rng();
    let num_events = Poisson::new(tmax*lambda).sample(&mut rng) as usize;
    let mut events = Array1::<f64>::zeros((num_events,));
    
    events.par_mapv_inplace(|_| {
        // get reference to local thread rng
        let mut rng = thread_rng();
        let u = Uniform::new(0.0, tmax);
        u.sample(&mut rng)
    });
    events
}

/// Simulate a Poisson process with variable intensity.
pub fn variable_poisson<F>(tmax: f64, lambda: F, max_lambda: f64) -> Array2<f64>
where F: Fn(f64) -> f64 + Send
{
    // Number of events before thinning
    let mut rng = thread_rng();
    let num_events = Poisson::new(tmax*max_lambda).sample(&mut rng);

    // Get timestamp and intensity values of events distributed
    // according to a homogeneous Poisson process
    // and keep those who are under the intensity curve
    let events: Vec<Array1<f64>> = (0..num_events).into_iter().filter_map(|_| {
        let mut rng = thread_rng();
        let ut = Uniform::new(0.0, tmax);
        let ul = Uniform::new(0.0, max_lambda);
        let timestamp = ut.sample(&mut rng);
        let lambda_val = ul.sample(&mut rng);

        if lambda_val < lambda(timestamp) {
            Some(array![timestamp, lambda_val])
        } else {
            None
        }
    }).collect();

    let events_ref: Vec<ArrayView2<f64>> = events.iter().map(|v| v.view().into_shape((1,2)).unwrap()).collect();

    stack(Axis(0), events_ref.as_slice()).unwrap()
}

/// Simulate a time-dependent marked Hawkes process with an exponential kernel
/// by utilising the O(n) algorithm in [Dassios and Zhao's 2013 paper](http://eprints.lse.ac.uk/51370/1/Dassios_exact_simulation_hawkes.pdf).
/// This will borrow and consume the given `jumps` iterator, and will panic if it turns up empty.
/// index 0: timestamps, index 1: intensity, index 2: marks
pub fn hawkes_exponential<T>(tmax: f64, beta: f64, lambda0: f64, jumps: &mut T) -> Array2<f64>
where T: Iterator<Item = f64>
{
    let mut t = 0.0;
    let mut previous_t: f64;
    let mut last_lambda = lambda0;
    let mut result = Vec::<Array2<f64>>::new();

    while t < tmax {
        // variables U_1 and S_{k+1}^(1) from the paper @DassiosZhao13
        let u1 = random::<f64>();
        let s1 = -1.0/beta*u1.ln();
        let d = if last_lambda > lambda0 {
            1.0 + beta*u1.ln()/(last_lambda - lambda0)
        } else { ::std::f64::NEG_INFINITY };
        let s2 = -1.0/lambda0*random::<f64>().ln();

        previous_t = t;
        t += if d < 0.0 {
            s2
        } else {
            s1.min(s2)
        };

        last_lambda = lambda0 + (last_lambda - lambda0)*(-beta*(t-previous_t)).exp();

        if let Some(alpha) = jumps.next() {
            let new_event: Array2<f64> = array![[t, last_lambda, alpha]];
            result.push(new_event);
            last_lambda += alpha;
        } else {
            panic!("Not enough marks for the Hawkes process.");
        }
    }

    let events: Vec<ArrayView2<f64>> = result.iter().map(|v| v.view()).collect();

    stack(Axis(0), events.as_slice()).unwrap()
}
