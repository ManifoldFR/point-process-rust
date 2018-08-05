extern crate pointprocesses;
extern crate serde_json;

use pointprocesses::poisson_process;


fn main() {
    
    let tmax = 10.0;
    let lambda = 3.0;

    let events = poisson_process(tmax, lambda);

    println!("{:?}", events);

}
