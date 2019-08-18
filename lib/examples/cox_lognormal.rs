extern crate pointprocesses;


use pointprocesses::temporal::cox;
use pointprocesses::temporal::utils;

fn main() {
    let dt = 0.1;
    let n = 101 as usize;
    let wt = utils::simulate_brownian(dt, n);
    println!("{:?}", wt);
}
