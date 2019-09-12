//! A crate for modelling, simulation and statistical inference
//! of point processes.
pub mod spatial;
pub mod likelihood;
pub mod temporal;
pub mod estimators;

/// Time-dependent processes should be available in the crate root.
pub use self::temporal::*;
