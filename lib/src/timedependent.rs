/*!
This module implements a set of time-dependent point processes, such as Poisson or Hawkes processes, on the real half-line [0,∞[.
*/
use rand::prelude::*;
use rand::distributions::Uniform;
use rand::distributions::Poisson;
use rand::distributions::Exp;

use ndarray::stack;
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

use rayon::prelude::*;

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
pub fn variable_poisson<F>(tmax: f64, lambda: &F, max_lambda: f64) -> Array2<f64>
where F: Fn(f64) -> f64 + Send + Sync
{
    // Number of events before thinning
    let mut rng = thread_rng();
    let num_events = Poisson::new(tmax*max_lambda).sample(&mut rng);

    let lambda = std::sync::Arc::from(lambda);

    // Get timestamp and intensity values of events distributed
    // according to a homogeneous Poisson process
    // and keep those who are under the intensity curve
    let events: Vec<Array2<f64>> = (0..num_events)
            .into_par_iter().filter_map(|_| {
        let mut rng = thread_rng();
        let ut = Uniform::new(0.0, tmax);
        let ul = Uniform::new(0.0, max_lambda);
        let timestamp = ut.sample(&mut rng);
        let lambda_val = ul.sample(&mut rng);

        if lambda_val < lambda(timestamp) {
            Some(array![[timestamp, lambda_val]])
        } else {
            None
        }
    }).collect();

    if events.len() > 0 {
        let events_ref: Vec<ArrayView2<f64>> = events.iter().map(|v| v.view()).collect();
        stack(Axis(0), events_ref.as_slice()).unwrap()
    } else {
        Array2::<f64>::zeros((0,2))
    }
}

/// Simulate a time-dependent marked Hawkes process with an exponential kernel
/// by utilising the O(n) algorithm in [Dassios and Zhao's 2013 paper](http://eprints.lse.ac.uk/51370/1/Dassios_exact_simulation_hawkes.pdf).
/// This will borrow and consume the given `jumps` iterator, and will panic if it turns up empty.
/// index 0: timestamps, index 1: intensity, index 2: marks
pub fn hawkes_exponential<T>(tmax: f64, beta: f64, lambda0: f64, jumps: &mut T) -> Array2<f64>
where T: Iterator<Item = f64>
{
    let ref mut rng = thread_rng();
    let mut result = Vec::<Array2<f64>>::new();

    let mut expdist = Exp::new(lambda0);
    let mut s = expdist.sample(rng);
    result.push(array![[s, lambda0, 0.0]]);

    while let Some(alpha) = jumps.next() {
        let last_time = result[result.len()-1][[0,0]];
        let last_lbda = result[result.len()-1][[0,1]];
        
        expdist = Exp::new(last_lbda);
        s += expdist.sample(rng);

        if s > tmax {
            break;
        }

        let candidate_lambda = alpha + lambda0 + (last_lbda-lambda0)*(-beta*(s-last_time)).exp();
        let d = random::<f64>();
        if d*last_lbda < candidate_lambda {
            let new_event: Array2<f64> = array![[s, candidate_lambda, alpha]];
            result.push(new_event);
        }

    }

    if result.len() > 0 {
        let events: Vec<ArrayView2<f64>> = result.iter().map(|v| v.view()).collect();
        stack(Axis(0), events.as_slice()).unwrap()
    } else {
        Array2::<f64>::zeros((0,3))
    }
}
