extern crate pointprocesses;

#[macro_use]
extern crate ndarray;

use ndarray::prelude::*;

use pointprocesses::generalized;

fn main() {
    let lambda = 2.0;

    let bounds = array![[0.0,0.0], [1.0,1.0]];
    let domain = generalized::Rectangle::new(bounds);

    let events = generalized::poisson_process(lambda, &domain);

    println!("{:?}", events);

}

