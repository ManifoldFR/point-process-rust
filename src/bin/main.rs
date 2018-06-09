extern crate pointprocesses;

use pointprocesses::event::Event;
use pointprocesses::{poisson_process,variable_poisson};

fn main() {
    
    let tmax = 60.0;
    let lambda = 3.0;

    let f: fn(f64) -> f64 = |t| 1.0+t.sqrt();

    let variable_events: Vec<Event> = variable_poisson(tmax, f, 1.0);

    println!("{:#?}", variable_events);

}
