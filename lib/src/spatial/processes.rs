use super::domains::Domain;

use rand::prelude::*;
use rand::rngs::SmallRng;
use rand_distr::{Poisson, Distribution};

use ndarray::stack;
use ndarray::prelude::*;

static XORSHIFT_ERR: &str = "Unable to create XorShift rng from thread local rng";

/// A higher-dimensional homogeneous Poisson process, for parallepipedal domains.
pub fn poisson_process(lambda: f64, domain: &Domain) -> Array2<f64>
{
    let ref mut rng = thread_rng();
    let far = &domain.far;
    let close = &domain.close;

    // dimension of space
    let d = far.shape()[0];
    let area = (0..d).fold(1.0, |area, i| {
        area * (far[i] - close[i])
    });

    // get number of events to generate
    // events outside of the set will be rejected
    let fish = Poisson::new(lambda*area).unwrap();
    let num_events: u64 = fish.sample(rng);
    
    let mut srng = SmallRng::from_rng(rng).expect(XORSHIFT_ERR);

    let events: Vec<Array2<f64>> = (0..num_events).map(|_| {
        // generate a point inside the bounding box
        let mut ev: Array1<f64> = Array::zeros((d,));

        for i in 0..d {
            ev[i] = srng.gen_range(close[i], far[i]);
        }

        // if it's in, then keep it
        ev.into_shape((1,d)).unwrap()
    }).collect();

    let events_ref: Vec<ArrayView2<f64>> = events.iter().map(|ev| {
        ev.view()
    }).collect();

    stack(Axis(0), events_ref.as_slice()).unwrap()
}

/// Poisson process on a d-dimensional region with variable intensity, using a rejection sampling algorithm.
pub fn variable_poisson<F>(lambda: F, max_lambda: f64, domain: &Domain) -> Array2<f64>
    where F: Fn(&Array1<f64>) -> f64 + Sync + Send
{
    let close = &domain.close;
    let far = &domain.far;

    let d = close.shape()[0];
    let area = (0..d).fold(1.0, |area, i| {
        area * (far[i] - close[i])
    });

    // get number of events to generate
    // events outside of the set will be rejected
    let ref mut rng = thread_rng();
    let fish = Poisson::new(max_lambda*area).unwrap();
    let num_events: u64 = fish.sample(rng);
    
    let mut srng = rand::rngs::SmallRng::from_rng(rng).expect(XORSHIFT_ERR);

    let events: Vec<Array2<f64>> = (0..num_events).filter_map(|_| {
        // generate a point inside the bounding box
        let mut ev: Array1<f64> = Array::zeros((d,));
        let intens = max_lambda*random::<f64>();

        for i in 0..d {
            ev[i] = srng.gen_range(close[i], far[i]);
        }

        // if it's in, then keep it
        if intens < lambda(&ev) {
            Some(ev.into_shape((1,d)).unwrap())
        } else {
            None
        }
    }).collect();

    let events_ref: Vec<ArrayView2<f64>> = events.iter().map(|ev| {
        ev.view()
    }).collect();

    stack(Axis(0), &events_ref).unwrap()
}
