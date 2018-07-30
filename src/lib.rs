#![feature(extern_prelude)]

extern crate rand;
extern crate serde;
extern crate rayon;

#[macro_use]
extern crate ndarray;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod event;
pub mod generalized;
pub mod timedependent;

/// Time dependent processes are available from the crate root.
pub use timedependent::*;