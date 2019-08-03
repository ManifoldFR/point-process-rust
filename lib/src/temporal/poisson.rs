use crate::temporal::traits::*;
use rand::prelude::*;
use rand::distributions::Uniform;
use rand_distr::{Poisson, Distribution};

use ndarray::array;
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

use rayon::prelude::*;


/// Homogeneous, constant intensity Poisson process.
pub struct PoissonProcess {
    /// Process intensity.
    lambda: f64
}

impl PoissonProcess {
    pub fn new(lambda: f64) -> Self {
        PoissonProcess {
            lambda
        }
    }
}

impl DeterministicIntensity for PoissonProcess {
    fn intensity(self, _t: f64) -> f64 {
        self.lambda
    }
}

/// Poisson process with variable intensity.
pub struct VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    /// Upper bound on the intensity function of the process.
    max_lambda: f64,
    lambda: F
}

impl<F> DeterministicIntensity for VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    fn intensity(self, t: f64) -> f64 {
        let func = self.lambda;
        func(t)
    }
}

impl<F> VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    pub fn new(lambda: F, max_lambda: f64) -> Self {
        VariablePoissonProcess {
            max_lambda,
            lambda
        }
    }
}


impl TemporalProcess for PoissonProcess {
    fn sample(self, tmax: f64) -> TimeProcessResult {
        let lambda = self.lambda;
        let mut rng = thread_rng();
        let fish = Poisson::new(tmax * lambda).unwrap();
        let num_events: u64 = fish.sample(&mut rng);
        let mut events = Array1::<f64>::zeros((num_events as usize,));
        
        events.par_mapv_inplace(|_| {
            // get reference to local thread rng
            let mut rng = thread_rng();
            let u = Uniform::new(0.0, tmax);
            u.sample(&mut rng)
        });
        let mut intensities = Array1::<f64>::zeros(num_events as usize);
        for i in 0..num_events as usize {
            intensities[i] = lambda;
        }
        (events, intensities)
    }
}

impl<F> TemporalProcess for VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    fn sample(self, tmax: f64) -> TimeProcessResult {
        // Parallelized, multithreaded algorithm for sampling
        // from the process.

        let mut rng = thread_rng();
        let max_lambda = self.max_lambda;
        let lambda = self.lambda;
        let fish = Poisson::new(tmax*max_lambda).unwrap();
        let num_events: u64 = fish.sample(&mut rng);
        let num_events = num_events as usize;
        let lambda = std::sync::Arc::from(lambda);
        
        // Get timestamp and intensity values of events distributed
        // according to a homogeneous Poisson process
        // and keep those who are under the intensity curve
        let events: Vec<Array1<f64>> = (0..num_events)
                .into_par_iter().filter_map(|_| {
            let mut rng = thread_rng();
            let timestamp = rng.gen::<f64>()*tmax;
            let lambda_val = rng.gen::<f64>()*max_lambda;

            if lambda_val < lambda(timestamp) {
                Some(array![timestamp, lambda_val])
            } else {
                None
            }
        }).collect();

        let num_events = events.len();

        if num_events > 0 {
            let mut evnts = Array1::<f64>::zeros(num_events);
            let mut intens = Array1::<f64>::zeros(num_events);
            for i in 0..num_events {
                evnts[i] = events[i][0];
                intens[i] = events[i][1];

            }
            (evnts, intens)
        } else {
            (Array1::<f64>::zeros((0,)), Array1::<f64>::zeros((0,)))
        }
    }
}
