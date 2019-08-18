/* Cox processes
 *
 */
use crate::temporal::traits::*;
use rand::prelude::*;
use rand_distr::{StandardNormal};

use ndarray::array;
use ndarray::prelude::*;
use ndarray_parallel::prelude::*;

use rayon::prelude::*;


/// Lognormal Cox process.
pub struct LognormalCox {
    mu: f64,
    sigma: f64
}


impl StochasticIntensity for LognormalCox {}

impl TemporalProcess for LognormalCox {
    /// Algorithm: acceptance-rejection method
    /// Propose new event time, compute brownian increment,
    /// update intensity and accept/reject
    fn sample(&self, tmax: f64) -> TimeProcessResult {

        let x = array![0.];
        let y = array![0.];
        TimeProcessResult {
            timestamps: x,
            intensities: y
        }
    }
}


