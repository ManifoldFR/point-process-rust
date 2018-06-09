extern crate rand;

pub mod event;

use rand::prelude::*;
use rand::distributions::Poisson;

use event::Event;

pub fn gen_events(t: f64, lambda: f64) -> Vec<Event> {
    let mut rng = thread_rng();
    
    assert!(lambda >= 0.0);

    let num_events = Poisson::new(lambda).sample(&mut rng);

    let mut result = vec!();
    for _ in 0..num_events {
        let timestamp= t*rand::random::<f64>();
        result.push(Event::new(timestamp, "Donald"));
    };
    result
}