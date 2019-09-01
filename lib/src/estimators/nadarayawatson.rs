//! Implement the Nadaraya-Watson non-parametric regression estimator.
//! Useful for estimating the intensity of a non-homogeneous Poisson process.
use ndarray::prelude::*;

use rayon::prelude::*;

/// Homogeneous fixed-bandwidth Gaussian kernel,
/// of the form
/// $$
///     K_h(x, y) = \exp\left(
///     -\frac{(x-x_i)^2}{2h^2}
///     \right)
/// $$
#[derive(Debug,Clone,Copy)]
pub struct GaussianKernel {
    bandwidth: f64
}

impl GaussianKernel {
    pub fn new(bandwidth: f64) -> Self {
        GaussianKernel { bandwidth }
    }
}

/// Traits for Nadararya-Watson estimator kernels, of the form
/// $$
///     K_h(x, y) = D\left(\frac{|x-y|}{h}\right)
/// $$
pub trait NWKernel {
    fn eval(&self, x: f64, xi: f64) -> f64;
}

impl NWKernel for GaussianKernel {
    fn eval(&self, x: f64, xi: f64) -> f64 {
        let z = (x - xi) / self.bandwidth;
        (-z * z / 2.).exp()
    }
}

/// Return a prediction of the function at `x0`:
/// $$
/// \hat y_0 = 
/// \frac{\sum_{i=1}^p K_h(x_i, x_0) y_i}
/// {\sum_{i=1}^p K_h(x_i, x_0)}
/// $$
pub struct NWEstimator<T: NWKernel> {
    kernel: T,
    x_i: Option<Array1<f64>>,
    priors: Option<Array1<f64>>
}

impl<T: NWKernel> NWEstimator<T> {
    /// Perform prediction at `x0`.
    pub fn predict(&self, x0: f64) -> f64
    {
        let x_arr: &Array1<f64> = self.x_i.as_ref().expect("Regressor was not fitted.");
        let y_arr: &Array1<f64> = self.priors.as_ref().expect("Regressor was not fitted.");

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


