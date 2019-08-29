//! Implements the Hawkes jump process.
use super::traits::*;
use rand::prelude::*;

use ndarray::prelude::*;


/// Kernel for the Hawkes process.
pub trait Kernel {
    fn eval(&self, t: f64) -> f64;
}


pub struct Hawkes<T, K: Kernel> {
    background: T,
    kernel: K
}

impl<T, K: Kernel> Hawkes<T, K> {
    pub fn get_kernel(&self) -> &K {
        &self.kernel
    }
    pub fn get_background(&self) -> &T {
        &self.background
    }
}

/// Constant background intensity for the Hawkes process.
pub struct ConstBackground(f64);

impl DeterministicIntensity for ConstBackground {
    fn intensity(&self, _t: f64) -> f64 {
        self.0
    }
}

/// Deterministic background intensity
/// Holds 
pub struct DeterministicBackground<F>
where F: Fn(f64) -> f64 {
    max_lbda0: f64,
    func: F
}

impl<F> DeterministicIntensity for DeterministicBackground<F>
where F: Fn(f64) -> f64 {
    fn intensity(&self, t: f64) -> f64 {
        (self.func)(t)
    }
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

/// Kernel of sums of exponentials,
/// of the form `g(t) = sum(alpha_i * exp(-beta_i * t))`
pub struct SumExpKernel {
    num_exp: usize,
    alphas: Vec<f64>,
    betas: Vec<f64>
}

impl Kernel for SumExpKernel {
    fn eval(&self, t: f64) -> f64 {
        let alphabetazip = self.alphas.iter().zip(self.betas.iter());
        let mut res = 0.;
        for (alpha,beta) in alphabetazip {
            res += alpha * (-beta*t).exp();
        }
        res
    }
}


// POWER LAW HAWKES

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

pub type PowerLawHawkes = Hawkes<ConstBackground, PowerLawKernel>;

impl PowerLawHawkes {
    /// Create a new power law Hawkes model instance.
    pub fn new(alpha: f64, beta: f64, delta: f64, lambda0: f64) -> Self {
        // Set the kernel.
        let kernel = PowerLawKernel {
            alpha, beta, delta
        };
        Self {
            background: ConstBackground(lambda0), kernel
        }
    }
}

// EXPONENTIAL HAWKES

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
        simulate_hawkes_exp_const_bk(self, tmax)
    }
}

impl<F> Hawkes<DeterministicBackground<F>, ExpKernel>
where F: Fn(f64) -> f64 {
    pub fn new(alpha: f64, beta: f64, func: F, max_lbda0: f64) -> Self {
        let kernel = ExpKernel { alpha, beta };
        let background = DeterministicBackground {
            max_lbda0,
            func
        };
        Self {
            background, kernel
        }
    }
}

impl<F> TemporalProcess for Hawkes<DeterministicBackground<F>, ExpKernel>
where F: Fn(f64) -> f64 {
    fn sample(&self, tmax: f64) -> TimeProcessResult {
        simulate_hawkes_exp_var_bk(self, tmax)
    }
}

// NUMERICAL ALGORITHM

/// Simulate a trajectory of an exponential kernel Hawkes jump process,
/// using Ogata's algorithm (1982).
/// Variant for constant background intensity.
fn simulate_hawkes_exp_const_bk(model: &ExpHawkes, tmax: f64) -> TimeProcessResult {
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

fn simulate_hawkes_exp_var_bk<F>(model: &Hawkes<DeterministicBackground<F>, ExpKernel>, tmax: f64)
-> TimeProcessResult
where F: Fn(f64) -> f64 {
    let kernel = &model.kernel;
    let alpha = kernel.alpha;
    let beta = kernel.beta;
    let lambda0 = &model.background;  // background intensity
    let max_lbda0 = model.background.max_lbda0;

    let mut rng = thread_rng(); // random no. generator
    let mut timestamps = Vec::new();
    let mut intensities = Vec::new();

    // Algorithm: we compute the intensity for each candidate time
    // by updating the background intensity and excited parts separately

    // Running maximum process used to sample before
    // acception-rejection step
    let mut max_lbda = max_lbda0;
    let mut s = 0.;
    let mut cur_slbda = 0.;  // current self-exciting intensity

    while s < tmax {
        let u: f64 = rng.gen();
        // candidate next event time
        let ds = -1./max_lbda * u.ln();
        s += ds;
        // background intensity
        let cur_blbda = lambda0.intensity(s);
        // decay the self-exciting part
        cur_slbda = cur_slbda * (-beta * ds).exp();
        let cur_lbda = cur_blbda + cur_slbda;  // total intensity
        
        if s > tmax {
            // end sampling
            break;
        }

        // rejection sampling step
        let acceptance = cur_lbda / max_lbda;
        let d: f64 = rng.gen();
        if d < acceptance {
            // accept the candidate event time.
            cur_slbda += alpha; // add jump to self-exciting intens
            // record event time and intensity
            let cur_lbda = cur_blbda + cur_slbda;
            timestamps.push(s);
            intensities.push(cur_lbda);
        }
        max_lbda = max_lbda0 + cur_slbda;  // update max intensity
    }

    let timestamps = Array1::from_vec(timestamps);
    let intensities = Array1::from_vec(intensities);

    TimeProcessResult {
        timestamps, intensities
    }
}

