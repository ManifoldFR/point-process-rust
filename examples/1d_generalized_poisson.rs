extern crate pointprocesses;

#[macro_use]
extern crate ndarray;

use pointprocesses::generalized::*;

fn main() {
    let p = array![0.];
    let q = array![1.];
    let lambda = 10.;
    let ref domain = Rectangle::new(p, q);
    let evs = poisson_process(lambda, domain);
    println!("{:?}", evs);
}