extern crate rand;
extern crate serde;

#[macro_use]
extern crate ndarray;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod event;
pub mod generalized;
pub mod timedependent;

// Time dependent processes are reexported.
pub use timedependent::*;