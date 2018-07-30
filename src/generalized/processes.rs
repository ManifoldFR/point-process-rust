use generalized::domains::Set;

use ndarray::stack;
use ndarray::prelude::*;

use rand::thread_rng;
use rand::distributions::Uniform;
use rand::distributions::Poisson;
use rand::prelude::*;

use std::sync::Arc;
use rayon::prelude::*;

/// A higher-dimensional homogeneous Poisson process.
pub fn poisson_process<T>(lambda: f64, domain: &T) -> Array2<f64> 
    where T: Set {
    let bounds = domain.bounding_box();
    let mut area = 1.0;

    // dimension of space
    let d = bounds.shape()[1];

    for i in 0..d {
        area *= bounds[[1,i]] - bounds[[0,i]];
    }

    // get number of events to generate
    // events outside of the set will be rejected
    let ref mut rng = thread_rng();
    let num_events = Poisson::new(lambda*area).sample(rng) as usize;

    let mut res = unsafe {
        Array::uninitialized((1,d))
    };
    
    for _ in 0..num_events {
        // generate a point inside the bounding box
        let mut ev = Array::zeros((d,));

        for i in 0..d {
            ev[i] = rng.sample(Uniform::new(bounds[[0,i]], bounds[[1,i]]));
        }

        // if it's in, then keep it
        if domain.contains(&ev) {
            res = stack(
                Axis(0), 
                &[res.view(), ev.into_shape((1,d)).unwrap().view()]
                ).unwrap();
        }
    }

    res
}

/// Poisson process on a d-dimensional region with variable intensity, using a rejection sampling algorithm.
pub fn variable_poisson<F, T>(lambda: F, max_lambda: f64, domain: &T) -> Array2<f64>
where F: Fn(&Array1<f64>) -> f64 + Sync + Send,
      T: Set
{
    let bounds = domain.bounding_box();
    let mut area = 1.0;

    let d = bounds.shape()[1];

    for i in 0..d {
        area *= bounds[[1,i]] - bounds[[0,i]];
    }

    // get number of events to generate
    // events outside of the set will be rejected
    let ref mut rng = thread_rng();
    let num_events = Poisson::new(max_lambda*area).sample(rng) as usize;

    let mut res = unsafe {
        Array::uninitialized((1,d))
    };
    
    for _ in 0..num_events {
        // generate a point inside the bounding box
        let mut ev = Array::zeros((d,));
        let intens = max_lambda*random::<f64>();

        for i in 0..d {
            ev[i] = rng.sample(Uniform::new(bounds[[0,i]], bounds[[1,i]]));
        }

        // if the point lies in the domain and the simulated intensity
        // fits, then add it.
        if domain.contains(&ev) && intens < lambda(&ev) {
            res = stack![
                Axis(0), 
                res.view(),
                ev.view().into_shape((1,d)).unwrap()
            ];
        }
    }

    res
}

