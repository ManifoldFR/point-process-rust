use rand::thread_rng;
use rand::distributions::Uniform;
use rand::distributions::Poisson;
use rand::prelude::*;

use ndarray::prelude::*;


/// Implement this trait to provide a method to check whether or not a vector lies in `self`.
pub trait Set {
    fn contains(&self, p: &Array<f64, Ix1>) -> bool;

    /// returns a bounding box for the set
    /// useful for Monte Carlo estimations of the area
    /// and also for point process simulation by rejection
    fn bounding_box(&self) -> Array<f64, Ix2>;
}

/// This trait must be implemented for elements of the Borel set, i.e. any measurable part in n-dimensional real space.
pub trait Measurable: Set {
    fn measure(&self) -> f64;
}


/// General n-dimensional hyperrectangle
pub struct Rectangle {
    /// `bounds[0]` is the closer extremety of the hyperrectangle
    /// and `bounds[1]` is the far point.
    bounds: Array<f64, Ix2>
}

impl Rectangle {
    pub fn new(bounds: Array<f64, Ix2>) -> Rectangle {

        assert_eq!(bounds.shape()[0], 2);

        Rectangle {
            bounds
        }
    }
}


impl Set for Rectangle {
    fn contains(&self, p: &Array<f64, Ix1>) -> bool {
        let bounds = &self.bounds;

        assert_eq!(p.len(), bounds.shape()[1]);
        
        // check if p is further away than the closer point
        let further = bounds.slice(s![0,..]).iter().zip(p.iter())
            .fold(true, |acc: bool, (v,w)| {
                acc & (w > v)
            });
        
        // check if p is closer than the far point
        let closer = bounds.slice(s![1,..]).iter().zip(p.iter())
            .fold(true, |acc: bool, (v,w)| {
                acc & (w < v)
            });

        // if both conditions are true then we're in the rectangle
        further & closer
    }

    fn bounding_box(&self) -> Array<f64, Ix2> {
        self.bounds.clone()
    }
}

impl Measurable for Rectangle {
    
    fn measure(&self) -> f64 {
        let bounds = &self.bounds;

        let mut result = 1.0;

        let n: usize = bounds.shape()[1];

        for i in 0..n {
            result *= bounds[[1,i]] - bounds[[0,i]];
        }

        result
    }
}

/// A higher-dimensional homogeneous Poisson process.
pub fn poisson_process<T>(lambda: f64, domain: &T) -> Array<f64, Ix2> 
    where T: Measurable {
    let area: f64 = domain.measure();
    let bounds = domain.bounding_box();
    let d: usize = bounds.shape()[1];

    let ref mut rng = thread_rng();
    let num_events = Poisson::new(lambda*area).sample(rng) as usize;

    let mut res = Array::zeros((num_events, d));
    
    let mut counter = 0_usize;
    while counter < num_events {

        // generate a point inside the bounding box
        let mut ev = Array::zeros((d,));

        for i in 0..d {
            ev[i] = rng.sample(Uniform::new(bounds[[0,i]], bounds[[1,i]]));
        }

        // if it's in, then keep it
        if domain.contains(&ev) {
            res.slice_mut(s![counter,..]).assign(&ev);
            counter += 1;
        }

    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_test() {
        let bounds = array![[0.0, 1.0], [1.0, 4.0]];
        
        let rect = Rectangle::new(bounds);
        assert_eq!(rect.measure(), 3.0);

        let p = array![0.5, 1.5];
        assert!(rect.contains(&p));

        let p = array![-1.0,2.0];
        assert!(!rect.contains(&p));

    }
}

