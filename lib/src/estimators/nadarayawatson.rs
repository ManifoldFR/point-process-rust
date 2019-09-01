//! Implement the Nadaraya-Watson non-parametric regression estimator.
//! Useful for estimating the intensity of a non-homogeneous Poisson process.
use ndarray::prelude::*;


/// Homogeneous fixed-bandwidth Gaussian kernel,
/// of the form
/// $$
///     K_h(x, x') = \exp\left(
///     -\frac{(x-x')^2}{2h^2}
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

/// Non-parametric regression kernel, of the form
/// $$
///     K_h(x, x') = D\left(\frac{|x-x'|}{h}\right)
/// $$
pub trait RegKernel {
    fn eval(&self, x: f64, xi: f64) -> f64;
}

impl RegKernel for GaussianKernel {
    fn eval(&self, x: f64, xi: f64) -> f64 {
        let z = (x - xi) / self.bandwidth;
        (-z * z / 2.).exp()
    }
}


/// Nadaraya-Watson nonparametric estimator using a weighted
/// kernel average.
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


