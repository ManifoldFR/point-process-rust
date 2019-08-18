use ndarray::prelude::*;

/// Result type for temporal processes.
/// Event timestamps and values of the intensity
pub struct TimeProcessResult {
    pub timestamps: Array1<f64>,
    pub intensities: Array1<f64>
}


/// Time-dependent point process.
pub trait TemporalProcess {
    /// Sample a sequence of events of the process.
    /// Returns: event timestamps and intensity process.
    fn sample(&self, tmax: f64) -> TimeProcessResult;
}

use std::fmt;
impl fmt::Debug for TimeProcessResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.debug_struct("ProcessTraj")
            .field("timestamps", &self.timestamps)
            .field("intensities", &self.intensities)
            .finish()
    }
}

/// Indicates the point process has a deterministic intensity process.
pub trait DeterministicIntensity {
    fn intensity(self, t: f64) -> f64;
}

/// Indicates the process has a stochastic intensity process;
pub trait StochasticIntensity {}
