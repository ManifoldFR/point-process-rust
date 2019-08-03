use ndarray::prelude::*;

/// Result type for temporal processes.
/// Event timestamps and values of the intensity
pub type TimeProcessResult = (Array1<f64>, Array1<f64>);


/// Time-dependent point process.
pub trait TemporalProcess {
    /// Sample a sequence of events of the process.
    /// Returns: event timestamps and intensity process.
    fn sample(self, tmax: f64) -> TimeProcessResult;
}

/// Indicates the point process has a deterministic intensity process.
pub trait DeterministicIntensity {
    fn intensity(self, t: f64) -> f64;
}
