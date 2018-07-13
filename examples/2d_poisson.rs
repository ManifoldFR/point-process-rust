extern crate pointprocesses;
extern crate gnuplot;

#[macro_use]
extern crate ndarray;

use gnuplot::{Figure,Caption,Color,PointSymbol,PointSize};

use pointprocesses::generalized;
use pointprocesses::generalized::Set;

fn main() {
    
    square_example();

    circle_example();

}

fn square_example() {
    let lambda = 20.0;

    let close = array![0.0,0.0];
    let far = array![1.0,1.0];
    let domain = generalized::Rectangle::new(close, far);

    let events = generalized::poisson_process(lambda, &domain);

    // println!("{:?}", events);
    println!("Bounding box: {:?}", domain.bounding_box());
    println!("Simulated {} events.", events.shape()[0]);

    let mut fg = Figure::new();

    fg.axes2d()
        .points(
            &events.slice(s![..,0]),
            &events.slice(s![..,1]),
            &[
                Caption("Events"),
                Color("red"),
                PointSymbol('O'),
                PointSize(0.8)
            ]
        );
    
    fg.echo_to_file("2d_poisson_square.gnuplot");
    fg.show();
}

fn circle_example() {
    let lambda = 100.0;

    let center = array![1.0,1.0];
    let radius = 1.0;
    let domain = generalized::Ball::new(center, radius);

    let events = generalized::poisson_process(lambda, &domain);

    // not printing this, the stdout gets flooded with all these evts
    // println!("{:?}", events);
    println!("Bounding box: {:?}", domain.bounding_box());
    println!("Simulated {} events.", events.shape()[0]);

    let mut fg = Figure::new();

    fg.axes2d()
        .points(
            &events.slice(s![..,0]),
            &events.slice(s![..,1]),
            &[
                Caption("Events"),
                Color("red"),
                PointSymbol('O'),
                PointSize(0.8)
            ]
        );
    
    fg.echo_to_file("2d_poisson_circle.gnuplot");
    fg.show();
}
