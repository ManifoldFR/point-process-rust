extern crate rand;
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod event;
pub mod generalized;

use rand::prelude::*;
use rand::distributions::Poisson;

use event::Event;

/// Simulates a homogeneous, constant-intensity Poisson process.
pub fn poisson_process(tmax: f64, lambda: f64) -> Vec<Event> {
    let mut rng = thread_rng();
    
    /// A poisson process cannot have negative intensity.
    assert!(lambda >= 0.0);

    let num_events = Poisson::new(tmax*lambda).sample(&mut rng);

    let mut result = vec![];
    for _ in 0..num_events {
        let timestamp= tmax*random::<f64>();
        result.push(Event::new(timestamp, lambda));
    };
    result
}

/// Simulate a Poisson process with variable intensity.
pub fn variable_poisson(tmax: f64, lambda: fn(f64) -> f64, max_lambda: f64) -> Vec<Event> {
    let mut rng = thread_rng();

    // Number of events before thinning
    let num_events = Poisson::new(tmax*max_lambda).sample(&mut rng);

    let mut result = vec![];

    // Get timestamp and intensity values of events distributed
    // according to a homogeneous Poisson process
    // and keep those who are under the intensity curve
    for _ in 0..num_events {
        let timestamp = random::<f64>()*tmax;
        let lambda_val = random::<f64>()*max_lambda;

        if lambda_val < lambda(timestamp) {
            let mut event = Event::new(timestamp, lambda_val);
            result.push(event);
        }
    }

    result
}

/// Simulate a Hawkes process with an exponential kernel
/// by utilising the linear time-complexity algorithm in [DassiosZhao13](http://eprints.lse.ac.uk/51370/1/Dassios_exact_simulation_hawkes.pdf)
pub fn hawkes_exponential(tmax: f64, alpha: f64, beta: f64, lambda0: f64) -> Vec<Event> {

    let mut t = 0.0;
    let mut previous_t: f64;
    let mut last_lambda = lambda0;
    let mut result: Vec<Event> = vec![];

    while t < tmax {
        let u0 = random::<f64>();
        let s0 = -1.0/lambda0*u0.ln();

        let d = if last_lambda > lambda0 {
            1.0 + beta*u0.ln()/(last_lambda - lambda0)
        } else { std::f64::NEG_INFINITY };
        
        let s1: f64;
        
        previous_t = t;

        
        t += if d > 0.0 {
            s1 = -1.0/lambda0*random::<f64>().ln();
            if s0 <= s1 {
                s0
            } else {
                s1
            }
        } else {
            s0
        };

        if t > tmax {
            break;
        }

        last_lambda = lambda0 + alpha + (last_lambda - lambda0)*(-beta*(t-previous_t)).exp();

        let new_event = Event::new(t, last_lambda);
        
        result.push(new_event);
    }

    result
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_serialize() {
        let event = Event::new(42.0, 15.02);


        let event_serialized = serde_json::to_string_pretty(&event).unwrap();

        println!("{}", event_serialized);
    }
}
