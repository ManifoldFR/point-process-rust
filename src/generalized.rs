use rand::thread_rng;
use rand::distributions::Poisson;
use rand::prelude::*;

use ndarray as na;
use ndarray::prelude::*;

type Scalar = na::OwnedRepr<f64>;

/// General n-dimensional hyperrectangle
struct Rectangle<D: Dimension>(ArrayBase<Scalar, D>);

pub trait Set<D: Dimension> {
    fn contains(&self, x: ArrayBase<Scalar,D>) -> bool
        where D: Dimension;
}

/// This trait must be implemented for elements of the Borel set, i.e. any measurable part in n-dimensional real space.
pub trait Measurable<D: Dimension>: Set<D> {
    fn measure(&self) -> f64;
}

impl<D> Set<D> for Rectangle<D> where D: Dimension {
    fn contains(&self, p: ArrayBase<Scalar,D>) -> bool {
        let bounds = &self.0;

        assert_eq!(bounds.len() % 2, 0);
        let dim = bounds.len()/2;

        let p_dim = p.dim();

        


        true

    }
}

impl<D: Dimension> Measurable<D> for Rectangle<D> {
    
    fn measure(&self) -> f64 {
        let bounds = &self.0;

        assert_eq!(bounds.len() % 2, 0);
        let dim = bounds.len()/2;

        let mut result = 1.0;

        result
    }
}

pub fn poisson_process<T,D>(lambda: f64, domain: T) -> ArrayBase<Scalar, Dim<[usize; 1]>> 
    where T: Measurable<D>,
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
        let bounds = array![[0.0, 1.0], [1.0, 3.0]];
        let rect = Rectangle(bounds);

        assert_eq!(rect.measure(), 2.0);

    }
}
