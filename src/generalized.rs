use rand::thread_rng;
use rand::distributions::Poisson;
use rand::prelude::*;

use ndarray as na;
use ndarray::prelude::*;

/// General n-dimensional hyperrectangle
struct Rectangle(Vec<f64>);

/// This trait must be implemented for elements of the Borel set, i.e. any measurable part in n-dimensional real space.
pub trait Measurable {
    fn measure(&self) -> f64;
}


impl Measurable for Rectangle {
    fn measure(&self) -> f64 {
        let bounds = &self.0;

        assert_eq!(bounds.len() % 2, 0);
        let dim = bounds.len()/2;

        let mut result = 1.0;
        for i in 0..dim {
            result *= bounds[2*i+1] - bounds[2*i];
        }

        result
    }
}

pub fn poisson_process<T,D>(lambda: f64, domain: T) -> ArrayBase<na::OwnedRepr<f64>, na::Dim<[usize; 1]>> where T: Measurable,
          D: Dimension {
    let area: f64 = domain.measure();
    let ref mut rng = thread_rng();
    let num_events = Poisson::new(lambda*area).sample(rng);

    let res = array![0.0,0.0];
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_test() {
        let bounds = vec![0.0, 1.0, 1.0, 3.0];
        let rect = Rectangle(bounds);

        assert_eq!(rect.measure(), 2.0);

    }
}
