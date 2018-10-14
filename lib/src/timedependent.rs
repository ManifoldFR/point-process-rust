/*!
 *This module implements a set of time-dependent point processes, such as Poisson or
 *Hawkes processes on the real half-line [0,∞[.
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
        let timestamp = rng.gen::<f64>()*tmax;
        let lambda_val = rng.gen::<f64>()*max_lambda;

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

/// Simulate a time-dependent marked Hawkes process with an exponential kernel.
/// This will borrow and consume the given `jumps` iterator, and will panic if it turns up empty.
/// index 0: timestamps, index 1: intensity, index 2: marks
pub fn hawkes_exponential<T>(tmax: f64, decay: f64, lambda0: f64, jumps: &mut T) -> Array2<f64>
where T: Iterator<Item = f64>
{
    let mut rng = thread_rng();
    let mut result = Vec::<Array2<f64>>::new();
    let mut prev_t: f64; // previous event time
    // compute a first event time, occuring as a standard poisson process
    // of intensity lambda0
    let mut expdist = Exp::new(lambda0);
    let mut s: f64 = expdist.sample(&mut rng);
    let alpha = jumps.next().unwrap();
    let mut cur_lambda = lambda0 + alpha*decay;
    prev_t = s; // record first event time
    result.push(array![[s, cur_lambda, alpha*decay]]);
    let mut lbda_max = cur_lambda;

    while let Some(alpha) = jumps.next() {
        expdist = Exp::new(lbda_max);
        // candidate time
        s += expdist.sample(&mut rng);
        if s > tmax {
            // time window is over, finish simulation loop
            break;
        }

        // compute process intensity at time s
        let increment = (-decay*(s-prev_t)).exp();
        cur_lambda = lambda0 + (cur_lambda-lambda0)*increment;

        // rejection sampling step
        let d = rng.gen::<f64>();
        if d < cur_lambda/lbda_max {
            // there was an event
            prev_t = s; // update last event time
            cur_lambda = cur_lambda + alpha*decay; // boost the intensity with the jump
            let event_vec: Array2<f64> = array![[s, cur_lambda, alpha*decay]];
            result.push(event_vec); // add the new state vector
        }
        lbda_max = cur_lambda; // update the max. intensity used for rejection sampling with the current intensity
    }

    if result.len() > 0 {
        let events: Vec<ArrayView2<f64>> = result.iter().map(|v| v.view()).collect();
        stack(Axis(0), &events).unwrap()
    } else {
        Array2::<f64>::zeros((0,3))
    }
}
