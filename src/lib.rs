extern crate rand;

pub mod event;

use rand::prelude::*;
use rand::distributions::Uniform;
use rand::distributions::Poisson;

use event::Event;

pub fn poisson_process(t: f64, lambda: f64) -> Vec<Event> {
    let mut rng = thread_rng();
    
    assert!(lambda >= 0.0);

    let num_events = Poisson::new(lambda).sample(&mut rng);

    let mut result = vec!();
    for _ in 0..num_events {
        let timestamp= t*random::<f64>();
        result.push(Event::new(timestamp));
    };
    result
}

/// Uses the Lewis thinning algorithm to simulate a variable Poisson process
pub fn variable_poisson(t: f64, lambda: fn(f64) -> f64, max_lambda: f64) -> Vec<Event> {
    let mut rng = thread_rng();

    // Number of events before thinning
    let num_events = Poisson::new(max_lambda).sample(&mut rng);

    let mut result = vec!();

    for _ in 0..num_events {
        let timestamp = random::<f64>()*t;
        let lambda_val = random::<f64>()*max_lambda;

        if lambda_val < lambda(timestamp) {
            result.push(
                Event::new(timestamp)
            );
        }
    }

    result
}
