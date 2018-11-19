/*!
 *This module implements a set of time-dependent point processes, such as Poisson or
 *Hawkes processes on the real half-line [0,∞[.
 */
use rand::prelude::*;
use rand::distributions::Uniform;
use rand::distributions::Poisson;

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
    let mut rng = thread_rng(); // random no. generator
    let mut result = Vec::<Array2<f64>>::new();
    // compute a first event time, occuring as a standard poisson process
    // of intensity lambda0
    let mut s = -1.0/lambda0*rng.gen::<f64>().ln();
    let alpha = jumps.next().unwrap();
    let mut cur_lambda = lambda0 + alpha;
    result.push(array![[s, cur_lambda, alpha]]);
    let mut lbda_max = cur_lambda;

    while let Some(alpha) = jumps.next() {
        let u: f64 = rng.gen();
        // candidate time
        let mut ds = -1.0/lbda_max*u.ln();
        // compute process intensity at new time s + ds
        // by decaying over the interval [s, s+ds]
        cur_lambda = lambda0 + (cur_lambda-lambda0)*(-decay*ds).exp();
        s += ds; // update s
        if s > tmax {
            // time window is over, finish simulation loop
            break;
        }

        // rejection sampling step
        let d: f64 = rng.gen();
        if d < cur_lambda/lbda_max {
            // accept the event
            cur_lambda = cur_lambda + alpha; // boost the intensity with the jump
            result.push(array![[s, cur_lambda, alpha]]); // add the new state vector
        }
        // update the intensity upper bound
        lbda_max = cur_lambda; 
    }

    if result.len() > 0 {
        let events: Vec<ArrayView2<f64>> = result.iter().map(|v| v.view()).collect();
        stack(Axis(0), &events).unwrap()
    } else {
        Array2::<f64>::zeros((0,3))
    }
}
