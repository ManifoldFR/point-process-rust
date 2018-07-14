use ndarray::stack;
use ndarray::prelude::*;

use rand::thread_rng;
use rand::distributions::Uniform;
use rand::distributions::Poisson;
use rand::prelude::*;



/// `structs` implementing this trait represent regions in n-dimensional space.
pub trait Set {
    /// Returns whether or not a given vector `p` lies in the instance.
    fn contains(&self, p: &Array1<f64>) -> bool;

    /// Returns a bounding box for the set.
    /// This function is used for point process simulation by rejection, but can also be used to implement a Monte Carlo estimation of the set"s measure.
    fn bounding_box(&self) -> Array2<f64>;
}

/// General n-dimensional hyperrectangle.
pub struct Rectangle(Array1<f64>, Array1<f64>);

impl Rectangle {
    /// `close` is the point with smaller coordinates,
    /// `far` is the further point delimiting the rectangle,
    /// with the largest coordinates.
    pub fn new(close: Array1<f64>, far: Array1<f64>) -> Rectangle {
        Rectangle(close, far)
    }
}


impl Set for Rectangle {
    fn contains(&self, p: &Array1<f64>) -> bool {
        assert_eq!(p.len(), self.0.shape()[0]);
        
        // check if p is further away than the closer point
        let further = self.0.iter().zip(p.iter())
            .fold(true, |acc: bool, (v,w)| {
                acc & (w > v)
            });
        
        // check if p is closer than the far point
        let closer = self.1.iter().zip(p.iter())
            .fold(true, |acc: bool, (v,w)| {
                acc & (w < v)
            });

        // if both conditions are true then we're in the rectangle
        further & closer
    }

    fn bounding_box(&self) -> Array<f64, Ix2> {
        let mut result = unsafe {
            Array::uninitialized((self.0.shape()[0], 2))
        };
        
        result.slice_mut(s![0,..]).assign(&self.0);
        result.slice_mut(s![1,..]).assign(&self.1);
        result
    }
}

/// The n-dimensional ball.
pub struct Ball {
    center: Array1<f64>,
    radius: f64
}

impl Ball {
    pub fn new(center: Array1<f64>, radius: f64) -> Ball {
        assert!(radius > 0.0);

        Ball {
            center, radius
        }
    }
}

impl Set for Ball {
    fn contains(&self, p: &Array1<f64>) -> bool {
        let diff = &self.center - p;
        let distance = diff.dot(&diff).sqrt();
        distance <= self.radius
    }

    fn bounding_box(&self) -> Array<f64, Ix2> {
        // Dimension of current space
        let n = self.center.shape()[0];

        let mut res = unsafe {
            Array::uninitialized((n, 2))
        };

        for i in 0..n {
            res[[0,i]] = self.center[i] - self.radius;
            res[[1,i]] = self.center[i] + self.radius;
        }

        res
    }
}

/// A higher-dimensional homogeneous Poisson process.
pub fn poisson_process<T>(lambda: f64, domain: &T) -> Array2<f64> 
    where T: Set {
    let bounds = domain.bounding_box();
    let mut area = 1.0;

    let n = bounds.shape()[0];
    let d = bounds.shape()[1];

    for i in 0..n {
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
where F: Fn(&Array1<f64>) -> f64,
      T: Set
{
    let bounds = domain.bounding_box();
    let mut area = 1.0;

    let n = bounds.shape()[0];
    let d = bounds.shape()[1];

    for i in 0..n {
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
            res = stack(
                Axis(0), 
                &[res.view(), ev.into_shape((1,d)).unwrap().view()]
                ).unwrap();
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_test() {
        let close = array![0.0, 1.0];
        let far = array![1.0, 4.0];
        
        let rect = Rectangle::new(close, far);

        let p = array![0.5, 1.5];
        assert!(rect.contains(&p));

        let p = array![-1.0,2.0];
        assert!(!rect.contains(&p));

    }
}

