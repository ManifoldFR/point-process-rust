extern crate pointprocesses;
extern crate serde_json;

use pointprocesses::event::Event;
use pointprocesses::poisson_process;


fn main() {
    
    let tmax = 10.0;
    let lambda = 3.0;

    let events: Vec<Event> = poisson_process(tmax, lambda);

    println!("{}",
        serde_json::to_string_pretty(&events).unwrap()
    );

}
