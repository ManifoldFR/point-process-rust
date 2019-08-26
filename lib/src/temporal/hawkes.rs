//! Implements the Hawkes jump process.
use super::traits::*;
use rand::prelude::*;

use ndarray::array;
use ndarray::prelude::*;



pub trait Kernel {
    fn eval(&self, t: f64) -> f64;
}


pub struct Hawkes<T, K: Kernel> {
    background: T,
    kernel: K
}



/// Exponential kernel for the Hawkes process,
/// of the form `g(t) = alpha * exp(-beta*t)`
pub struct ExpKernel {
    alpha: f64,
    beta: f64
}

impl Kernel for ExpKernel {
    fn eval(&self, t: f64) -> f64 {
        self.alpha * (-self.beta * t).exp()
    }
}

/// Power law kernel
/// of the form `g(t) = alpha / pow(t, beta)`
pub struct PowerLawKernel {
    alpha: f64,
    beta: f64,
    delta: f64
}

impl Kernel for PowerLawKernel {
    fn eval(&self, t: f64) -> f64 {
        self.alpha / (self.delta + t).powf(self.beta)
    }
}

impl<T> Hawkes<T, PowerLawKernel> {
    pub fn new(alpha: f64, beta: f64, delta: f64, bk: T) -> Self {
        let kernel = PowerLawKernel {
            alpha, beta, delta
        };
        Self {
            background: bk, kernel
        }
    }
}

// EXPONENTIAL HAWKES

/// Constant background intensity
pub struct ConstBackground(f64);


/// Hawkes model with an exponential kernel.
pub type ExpHawkes = Hawkes<ConstBackground, ExpKernel>;

impl ExpHawkes {
    pub fn new(alpha: f64, beta: f64, lambda0: f64) -> Self {
        let kernel = ExpKernel {
            alpha, beta
        };
        let background = ConstBackground(lambda0);
        
        Self {
            background, kernel
        }
    }
}

impl TemporalProcess for ExpHawkes {
    fn sample(&self, tmax: f64) -> TimeProcessResult {
        simulate_hawkes_exp(self, tmax)
    }
}

// NUMERICAL ALGORITHM

/// Simulate a trajectory of an exponential kernel Hawkes jump process,
/// using Ogata's algorithm (1982).
fn simulate_hawkes_exp(model: &ExpHawkes, tmax: f64) -> TimeProcessResult {
    let kernel = &model.kernel;
    let alpha = kernel.alpha;
    let decay = kernel.beta;
    let lambda0 = model.background.0;
    
    let mut rng = thread_rng(); // random no. generator
    let mut timestamps = Vec::new();
    let mut intensities = Vec::new();
    // compute a first event time, occurring as a standard poisson process
    // of intensity lambda0
    let mut s = -1.0/lambda0*rng.gen::<f64>().ln();
    let mut cur_lambda = lambda0 + alpha;
    let mut lbda_max = cur_lambda;

    while s < tmax {
        let u: f64 = rng.gen();
        // candidate time
        let ds = -1.0/lbda_max*u.ln();
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
            timestamps.push(s);
            intensities.push(cur_lambda);
        }
        // update the intensity upper bound
        lbda_max = cur_lambda; 
    }

    let timestamps = Array1::from_vec(timestamps);
    let intensities = Array1::from_vec(intensities);

    TimeProcessResult {
        timestamps, intensities
    }
}

