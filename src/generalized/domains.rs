/*!
 * Traits and some base structs for use with n-dimensional processes.
 */
use ndarray::prelude::*;

/// Structs implementing this trait represent regions in n-dimensional space.
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
