#![feature(extern_prelude)]

extern crate rand;
extern crate serde;
extern crate rayon;

#[macro_use]
extern crate ndarray;
extern crate ndarray_parallel;

extern crate serde_json;

pub mod generalized;
pub mod timedependent;

/// Time dependent processes are available from the crate root.
pub use timedependent::*;