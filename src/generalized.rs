use rand::prelude::*;

/// General n-dimensional domain
pub enum Domain {
    Rectangle(Vec<f64>)
}

pub trait Measurable {
    fn measure(&self) -> f64;
}

impl Domain {

}

impl Measurable for Domain {
    fn measure(&self) -> f64 {
        match self {
            Domain::Rectangle(bounds) => {
                

                0.0
            }
        }
    }
}

pub fn poisson_process(lambda: f64, domain: Domain) -> f64 {
    let area: f64 = domain.measure();

    0.0
}
