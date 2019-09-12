//! Kernel structs.

/// Trait for non-parametric regression kernels of the form
/// $$
///     K_h(x, x') = D\left(\frac{|x-x'|}{h}\right)
/// $$
pub trait RegKernel {
    fn eval(&self, x: f64, xi: f64) -> f64;
}

/// Trait for kernel mass integrals.
pub trait RegKernelMass: RegKernel {
    /// Compute the mass of the kernel between `a` and `b` shifted by `x`
    /// $$
    ///     \int_a^b K_h(x - u)\\, du
    /// $$
    fn eval_mass(&self, x: f64, a: f64, b: f64) -> f64;
}

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

impl RegKernel for GaussianKernel {
    fn eval(&self, x: f64, xi: f64) -> f64 {
        let z = (x - xi) / self.bandwidth;
        (-z * z / 2.).exp()
    }
}

/// Fixed-bandwidth nearest-neighbor (or uniform) kernel,
/// $$
///     K_h(x, x') = \mathbf{1}_{|x - x'| < h}
/// $$
pub struct NearestNeighborKernel {
    bandwidth: f64
}

impl NearestNeighborKernel {
    pub fn new(bandwidth: f64) -> Self {
        Self { bandwidth }
    }
}

impl RegKernel for NearestNeighborKernel {
    fn eval(&self, x: f64, xi: f64) -> f64 {
        if (x - xi).abs() < self.bandwidth {
            1.0
        } else {
            0.0
        }
    }
}

impl RegKernelMass for NearestNeighborKernel {
    fn eval_mass(&self, x: f64, a: f64, b: f64) -> f64 {
        let h = self.bandwidth;
        let up = b.min(x + h);
        let dw = a.max(x - h);
        up - dw
    }
}

/// The Epanechnikov kernel is given by
/// $$
///     K_h(x, x') = D\left(
///     \frac{|x-x'|}{h}
///     \right)
/// $$
/// where $D(u) = \frac34(1-u^2)\mathbf{1}_{|u|\leq 1}$.
#[derive(Debug,Clone,Copy)]
pub struct EpanechnikovKernel {
    bandwidth: f64
}

impl EpanechnikovKernel {
    /// Instantiate a new Epanechnikov kernel.
    pub fn new(bandwidth: f64) -> Self {
        Self { bandwidth }
    }
}

impl RegKernel for EpanechnikovKernel {
    fn eval(&self, x: f64, xi: f64) -> f64 {
        if x.abs() > 1. {
            0.
        }
        else {
            let dx = (x - xi) / self.bandwidth;
            0.75 * (1.0 - dx * dx)
        }
    }
}
