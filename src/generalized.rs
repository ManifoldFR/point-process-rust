use rand::thread_rng;
use rand::distributions::Uniform;
use rand::distributions::Poisson;
use rand::prelude::*;

use ndarray::prelude::*;


/// `structs` implementing this trait represent regions in n-dimensional space,
/// and check if a vector lies in it.
pub trait Set {
    fn contains(&self, p: &Array<f64, Ix1>) -> bool;

    /// Returns a bounding box for the set.
    /// This function is useful for Monte Carlo estimations of the area
    /// and also for point process simulation by rejection.
    fn bounding_box(&self) -> Array<f64, Ix2>;
}

/// This trait must be implemented for elements of the Borel set, i.e. any measurable part in n-dimensional real space.
pub trait Measurable: Set {
    fn measure(&self) -> f64;
}


/// General n-dimensional hyperrectangle.
pub struct Rectangle(Array<f64, Ix1>, Array<f64, Ix1>);

impl Rectangle {
    /// `close` is the point with smaller coordinates,
    /// `far` is the further point delimiting the rectangle,
    /// with the largest coordinates.
    pub fn new(close: Array<f64, Ix1>, far: Array<f64, Ix1>) -> Rectangle {
        Rectangle(close, far)
    }
}


impl Set for Rectangle {
    fn contains(&self, p: &Array<f64, Ix1>) -> bool {
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

impl Measurable for Rectangle {
    fn measure(&self) -> f64 {
        let n: usize = self.0.shape()[0];

        let mut result = 1.0;
        for i in 0..n {
            result *= self.1[i] - self.0[i];
        }

        result
    }
}

pub struct Ball {
    center: Array<f64, Ix1>,
    radius: f64
}

impl Ball {
    pub fn new(center: Array<f64, Ix1>, radius: f64) -> Ball {
        assert!(radius > 0.0);

        Ball {
            center, radius
        }
    }
}

impl Set for Ball {
    fn contains(&self, p: &Array<f64, Ix1>) -> bool {
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

impl Measurable for Ball {
    fn measure(&self) -> f64 {
        use std::f64::consts::PI as PI;
        const MONTE_CARLO_STEPS: u32 = 1000000;

        let n = self.center.shape()[0];

        // compute the area, using the formulas for the first
        // few dimensions and then a Monte-Carlo method.
        match n {
            0 => 1.0,
            1 => 2.0*self.radius,
            2 => PI*self.radius.powi(2),
            3 => 4.0*PI*self.radius.powi(3)/3.0,
            4 => 0.5*PI.powi(2)*self.radius.powi(4),
            5 => 8.0*PI.powi(2)*self.radius.powi(5)/15.0,
            6 => PI.powi(3)*self.radius.powi(6)/6.0,
            7 => 16.0*PI.powi(3)*self.radius.powi(7)/105.0,
            8 => PI.powi(4)*self.radius.powi(8)/24.0,
            9 => 32.0*PI.powi(4)*self.radius.powi(9)/945.0,
            10 => PI.powi(5)*self.radius.powi(10)/120.0,
            _ => {
                let ref mut rng = thread_rng();
                let bounds = self.bounding_box();
                let mut count = 0;
                for _ in 0..MONTE_CARLO_STEPS {
                    let mut p = unsafe {
                        Array::uninitialized((n,))
                    };

                    for i in 0..n {
                        p[i] = rng.sample(Uniform::new(bounds[[0,i]], bounds[[1,i]]));
                    }

                    if self.contains(&p) {
                        count += 1;
                    }
                }
                count as f64/MONTE_CARLO_STEPS as f64
            }
        }

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

    let mut res = unsafe {
        Array::uninitialized((num_events, d))
    };
    
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
        let close = array![0.0, 1.0];
        let far = array![1.0, 4.0];
        
        let rect = Rectangle::new(close, far);
        assert_eq!(rect.measure(), 3.0);

        let p = array![0.5, 1.5];
        assert!(rect.contains(&p));

        let p = array![-1.0,2.0];
        assert!(!rect.contains(&p));

    }
}

