extern crate pointprocesses;
extern crate plotlib;
extern crate svg;

#[macro_use]
extern crate ndarray;

use std::fs;

use plotlib::style::{PointStyle, PointMarker};
use plotlib::view;
use plotlib::page;
use plotlib::repr::Scatter;

use ndarray::{Array1, Array2};

use pointprocesses::spatial::*;

static MARKER_COLOR: &str = "#C93E3E";


fn main() {
    fs::create_dir("lib/examples/images").unwrap_or_default();
    square_example();
    variable_circle_example();
}

/// The `plotlib` library expects vectors of tuples for input.
fn cast_to_tuples(arr: Array2<f64>) -> Vec<(f64,f64)> {
    let n = arr.shape()[0];
    (0..n).map(|i: usize| {
        let v = arr.slice(s![i,..]);
        (v[0], v[1])
    }).collect()
}

fn square_example() {
    let lambda = 200.0;

    let close = array![0.0,0.0];
    let far = array![1.0,1.0];
    let domain = Domain::new(close, far);

    let events = poisson_process(lambda, &domain);

    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let d = events.shape()[1];
    assert_eq!(d, 2);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data$
    let s1 = Scatter::from_slice(&data)
        .style(PointStyle::new()
            .marker(PointMarker::Circle) // setting the marker to be a square
            .colour(MARKER_COLOR)
            .size(2.5)); // and a custom colour 
    
    let v = view::ContinuousView::new()
        .add(s1)
        .x_range(0., 1.)
        .y_range(0., 1.)
        .x_label("x")
        .y_label("y");
    
    page::Page::single(&v)
        .dimensions(600, 600)
        .save("lib/examples/images/2d_poisson.rect.svg").unwrap();
}

fn variable_circle_example() {
    let close: Array1<f64> = array![0.,0.];
    let far = array![1.,1.];
    let domain = Domain::new(close, far);

    // the intensity function will be the distance to center
    let lambda = |v: &Array1<f64>| {
        use std::f64::consts::PI;
        let center = array![0.5,0.5];
        let radius = 1.;
        let v = v - &center;
        let distance = v.dot(&v).sqrt();
        let x = v[0];
        let y = v[1];
        let angle = y.atan2(x);
        let perturb =  0.04*(10.0*angle).cos();
        if distance <= radius {
            4000.0 * (6.0 * (distance + perturb) * PI).cos()
        } else {
            0.0
        }
    };

    let events = variable_poisson(lambda, 4000.0, &domain);

    // not printing this, the stdout gets flooded with all these evts
    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data
    let s = Scatter::from_slice(&data)
        .style(PointStyle::new()
            .marker(PointMarker::Circle)
            .colour(MARKER_COLOR)
            .size(1.2)); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = view::ContinuousView::new()
        .add(s)
        .x_range(0., 1.)
        .y_range(0., 1.)
        .x_label("x")
        .y_label("y");

    page::Page::single(&v)
        .dimensions(600, 600)
        .save("lib/examples/images/2d_poisson.variable.circle.svg").unwrap();
}
