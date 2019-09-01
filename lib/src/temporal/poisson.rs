//! Poisson processes.
use super::traits::*;
use rand::prelude::*;
use rand_distr::{Uniform, Poisson, Distribution};

use ndarray::array;
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

use rayon::prelude::*;


/// Homogeneous, constant intensity Poisson process.
/// The intensity of the process is given by the average
/// number of events between two instants:
/// $$
/// \lambda = \lim_{h\to 0}
/// \frac{\mathbb E[N_{t+h} - N_t]}{h}
/// $$
#[derive(Debug)]
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
    fn intensity(&self, _t: f64) -> f64 {
        self.lambda
    }
}

/// Poisson process with variable intensity.
/// The average number of events between $t$ and $t+dt$ is
/// $$ \mathbb{E}[ dN_t ] = \lambda(t) dt $$
#[derive(Debug)]
pub struct VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    /// Upper bound on the intensity function of the process.
    max_lambda: f64,
    /// Process intensity function.
    func: F
}

impl<F> VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    pub fn new(func: F, max_lambda: f64) -> Self {
        VariablePoissonProcess {
            max_lambda,
            func
        }
    }

    /// Get the max_lambda upper bound on the intensity.
    pub fn get_max_lambda(&self) -> f64 {
        self.max_lambda
    }
}

impl<F> DeterministicIntensity for VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    fn intensity(&self, t: f64) -> f64 {
        (self.func)(t)
    }
}


impl TemporalProcess for PoissonProcess {
    fn sample(&self, tmax: f64) -> TimeProcessResult {
        let lambda = self.lambda;
        let mut rng = thread_rng();
        let fish = Poisson::new(tmax * lambda).unwrap();
        let num_events: u64 = fish.sample(&mut rng);
        let num_events = num_events as usize;
        
        let mut events_vec: Vec<_> = (0..num_events).into_par_iter()
            .map(|_| {
                // get reference to local thread rng
                let mut rng = thread_rng();
                let u = Uniform::new(0.0, tmax);
                u.sample(&mut rng)
        }).collect();
        events_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let timestamps = Array1::<f64>::from_vec(events_vec);
        let mut intensities = Array1::<f64>::zeros(num_events as usize);
        for i in 0..num_events as usize {
            intensities[i] = lambda;
        }
        TimeProcessResult {
            timestamps, intensities
        }
    }
}

impl<F> TemporalProcess for VariablePoissonProcess<F>
where F: Fn(f64) -> f64 + Send + Sync
{
    fn sample(&self, tmax: f64) -> TimeProcessResult {
        // Parallelized, multithreaded algorithm for sampling
        // from the process.

        let mut rng = thread_rng();
        let max_lambda = self.max_lambda;
        let lambda = &self.func;
        let fish = Poisson::new(tmax*max_lambda).unwrap();
        let num_events: u64 = fish.sample(&mut rng);
        let num_events = num_events as usize;
        let lambda = std::sync::Arc::from(lambda);
        
        // Get timestamp and intensity values of events distributed
        // according to a homogeneous Poisson process
        // and keep those who are under the intensity curve
        let mut events: Vec<Array1<f64>> = (0..num_events)
                .into_par_iter().filter_map(|_| {
            let mut rng = thread_rng();
            let timestamp = rng.gen::<f64>()*tmax;
            let lambda_val = rng.gen::<f64>()*max_lambda;

            if lambda_val < lambda(timestamp) {
                Some(array![timestamp, lambda(timestamp)])
            } else {
                None
            }
        }).collect();
        events.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        let num_events = events.len();

        let mut timestamps = Array1::<f64>::zeros(num_events);
        let mut intensities = Array1::<f64>::zeros(num_events);
        if num_events > 0 {
            for i in 0..num_events {
                timestamps[i] = events[i][0];
                intensities[i] = events[i][1];

            }
        }
        TimeProcessResult { timestamps, intensities }
    }
}
