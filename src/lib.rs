extern crate rand;

pub mod event;

use rand::prelude::*;
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

/// Simulate a variable Poisson process
pub fn variable_poisson(t: f64, lambda: fn(f64) -> f64, max_lambda: f64) -> Vec<Event> {
    let mut rng = thread_rng();

    // Number of events before thinning
    let num_events = Poisson::new(max_lambda*t).sample(&mut rng);

    let mut result = vec!();

    // Get timestamp and intensity values of events distributed
    // according to a homogeneous Poisson process
    // and keep those who are under the intensity curve
    for _ in 0..num_events {
        let timestamp = random::<f64>()*t;
        let lambda_val = random::<f64>()*max_lambda;

        if lambda_val < lambda(timestamp) {
            let mut event = Event::new(timestamp);
            event.add_intensity(lambda_val);
            result.push(event);
        }
    }

    result
}
