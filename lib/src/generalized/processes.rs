use generalized::domains::Set;

use ndarray::stack;
use ndarray::prelude::*;

use rand::thread_rng;
use rand::distributions::Range;
use rand::distributions::Poisson;
use rand::prelude::*;

static XORSHIFT_ERR: &str = "Unable to create XorShift rng from thread local rng";

/// A higher-dimensional homogeneous Poisson process.
pub fn poisson_process<T>(lambda: f64, domain: &T) -> Array2<f64> 
    where T: Set
{
    let ref mut rng = thread_rng();
    let bounds = domain.bounding_box();

    // dimension of space
    let d = bounds.shape()[1];
    let area = (0..d).fold(1.0, |area, i| {
        area * (bounds[[1,i]] - bounds[[0,i]])
    });

    // get number of events to generate
    // events outside of the set will be rejected
    let num_events = Poisson::new(lambda*area).sample(rng) as usize;

    let mut srng = rand::rngs::SmallRng::from_rng(rng).expect(XORSHIFT_ERR);

    let events: Vec<Array2<f64>> = (0..num_events).filter_map(|_| {
        // generate a point inside the bounding box
        let mut ev: Array1<f64> = Array::zeros((d,));

        for i in 0..d {
            ev[i] = srng.sample(Range::new(bounds[[0,i]], bounds[[1,i]]));
        }

        // if it's in, then keep it
        if domain.contains(&ev) {
            Some(ev.into_shape((1,d)).unwrap())
        } else {
            None
        }
    }).collect();

    let events_ref: Vec<ArrayView2<f64>> = events.iter().map(|ev| {
        ev.view()
    }).collect();

    stack(Axis(0), events_ref.as_slice()).unwrap()
}

/// Poisson process on a d-dimensional region with variable intensity, using a rejection sampling algorithm.
pub fn variable_poisson<F, T>(lambda: F, max_lambda: f64, domain: &T) -> Array2<f64>
    where F: Fn(&Array1<f64>) -> f64 + Sync + Send,
          T: Set
{
    let bounds = domain.bounding_box();

    let d = bounds.shape()[1];
    let area = (0..d).fold(1.0, |area, i| {
        area * (bounds[[1,i]] - bounds[[0,i]])
    });

    // get number of events to generate
    // events outside of the set will be rejected
    let ref mut rng = thread_rng();
    let num_events = Poisson::new(max_lambda*area).sample(rng) as usize;

    let mut srng = rand::rngs::SmallRng::from_rng(rng).expect(XORSHIFT_ERR);

    let events: Vec<Array2<f64>> = (0..num_events).filter_map(|_| {
        // generate a point inside the bounding box
        let mut ev: Array1<f64> = Array::zeros((d,));
        let intens = max_lambda*random::<f64>();

        for i in 0..d {
            ev[i] = srng.sample(Range::new(bounds[[0,i]], bounds[[1,i]]));
        }

        // if it's in, then keep it
        if domain.contains(&ev) && intens < lambda(&ev) {
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

pub fn multivariate_hawkes<T>(domain: &T) -> Array2<f64>
where T: Set
{
    let bounds = domain.bounding_box();
    Array2::<f64>::zeros((2,3))
}