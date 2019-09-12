//! Implement the Nadaraya-Watson non-parametric regression estimator.
//! Useful for estimating the intensity of a non-homogeneous Poisson process.
use ndarray::prelude::*;

use super::kernels::*;


/// Nadaraya-Watson nonparametric estimator for functions using
/// a weighted kernel average.
/// The predictor at a point $x_0$ is given by:
/// $$
/// \hat y_0 = 
/// \frac{\sum_{i=1}^p K_h(x_i, x_0) y_i}
/// {\sum_{i=1}^p K_h(x_i, x_0)}
/// $$
pub struct NadWatEstimator<T: RegKernel> {
    kernel: T,
    x_i: Option<Array1<f64>>,
    y_i: Option<Array1<f64>>
}


impl<T: RegKernel> NadWatEstimator<T> {
    /// Return a new Nadaraya-Watson estimator.
    pub fn new(kernel: T) -> Self {
        Self { kernel, x_i: None, y_i: None }
    }

    pub fn fit(mut self, x_i: &Array1<f64>, y_i: &Array1<f64>) -> Self {
        self.x_i.get_or_insert(x_i.clone());
        self.y_i.get_or_insert(y_i.clone());
        self
    }

    /// Perform prediction at `x0`.
    pub fn predict(&self, x0: f64) -> f64
    {
        let x_arr: &Array1<f64> = self.x_i.as_ref().expect("Regressor was not fitted.");
        let y_arr: &Array1<f64> = self.y_i.as_ref().expect("Regressor was not fitted.");

        let kernel = &self.kernel;

        let zipped_i = x_arr.iter().zip(y_arr.iter());
        let numerator = zipped_i.fold(
            0., |acc, (x, y)| {
                acc + kernel.eval(x0, *x) * y
            }
        );
        let denom = x_arr.iter().fold(
            0., |acc, x| {
                acc + kernel.eval(x0, *x)
            }
        );

        numerator / denom
    }
}

/// Estimate the intensity function of an event sequence under a
/// variable Poisson model using a kernel smoother 
/// (see _A kernel method for smoothing point process data_ by P. Diggle).
/// The regressor is given by
/// $$
///     \hat\lambda(t) = e_h(t)^{-1}
///     \sum_i K_h(t - t_i)
/// $$
/// where $e_h(t) = \int_0^T K_h(t - u)\\, du$ is an edge-correction term.
pub struct SmoothingKernelIntensity<K: RegKernel> {
    event_times: Vec<Array1<f64>>,
    kernel: K
}

/// Intensity kernel estimator using a uniform kernel.
pub type UniformKernelIntensity = SmoothingKernelIntensity<NearestNeighborKernel>;

impl UniformKernelIntensity {
    pub fn new(bandwidth: f64) -> Self {
        let kernel = NearestNeighborKernel::new(bandwidth);
        let event_times = Vec::new();
        Self {
            event_times,
            kernel
        }
    }

    pub fn fit<T>(mut self, evts: Vec<T>) -> Self
    where T: Into<Array1<f64>> {
        self.event_times.reserve(evts.len());
        for e in evts {
            self.event_times.push(e.into())
        }

        self
    }

    pub fn predict(&self, x0: f64, tmax: f64) -> f64 {
        let kernel = &self.kernel;

        let edge_correct = 1. /  kernel.eval_mass(x0, 0., tmax);
        let num_seq = self.event_times.len();

        let sum: f64 = self.event_times.iter()
            .map(|seq| {
                seq.into_iter()
                    .fold(0., |acc, xi| {
                        acc + kernel.eval(x0, *xi)
                    })
            }).sum();


        edge_correct * sum / num_seq as f64
    }

}
