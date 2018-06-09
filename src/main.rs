extern crate pointprocesses;

use pointprocesses::event::Event;
use pointprocesses::gen_events;

fn main() {
    
    let lambda = 3.0;

    let events: Vec<Event> = gen_events(30.0, lambda);

    println!("{:#?}", events);

}
