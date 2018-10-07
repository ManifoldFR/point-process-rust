extern crate pointprocesses;
extern crate plotlib;
extern crate svg;

#[macro_use]
extern crate ndarray;

use std::fs;

use plotlib::style::{Point, Marker};
use plotlib::view;
use plotlib::page;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use ndarray::{Array1, Array2};

use pointprocesses::generalized::*;

static MARKER_COLOR: &str = "#C93E3E";

fn main() {
    fs::create_dir("examples/images").unwrap_or_default();
    square_example();
    circle_example();
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
    let domain = Rectangle::new(close, far);

    let events = poisson_process(lambda, &domain);

    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let d = events.shape()[1];
    assert_eq!(d, 2);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data$
    let s1 = Scatter::from_slice(&data)
        .style(scatter::Style::new()
            .marker(Marker::Circle) // setting the marker to be a square
            .colour(MARKER_COLOR)
            .size(2.5)); // and a custom colour 
    
    let v = view::ContinuousView::new()
        .add(&s1)
        .x_range(0., 1.)
        .y_range(0., 1.)
        .x_label("x")
        .y_label("y");
    
    page::Page::single(&v)
        .dimensions(600, 600)
        .save("examples/images/2d_poisson.rect.svg").unwrap();
}


fn circle_example() {
    let lambda = 400.0;

    let center = array![1.0,1.0];
    let radius = 1.0;
    let domain = Ball::new(center, radius);

    let events = poisson_process(lambda, &domain);

    // not printing this, the stdout gets flooded with all these evts
    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data
    let s = Scatter::from_slice(&data)
        .style(scatter::Style::new()
            .marker(Marker::Circle) // setting the marker to be a square
            .colour(MARKER_COLOR)
            .size(2.0)); // and a custom colour 
    
    // The 'view' describes what set of data is drawn
    let v = view::ContinuousView::new()
        .add(&s)
        .x_range(0., 2.)
        .y_range(0., 2.)
        .x_label("x")
        .y_label("y");

    page::Page::single(&v)
        .dimensions(600, 600)
        .save("examples/images/2d_poisson.circle.svg").unwrap();
}


fn variable_circle_example() {
    let center: Array1<f64> = array![0.,0.];
    let radius = 1.0;
    let domain = Ball::new(center, radius);

    // the intensity function will be the distance to center
    let lambda = |v: &Array1<f64>| {
        use std::f64::consts::PI;
        let distance = v.dot(v).sqrt();
        let x = v[0];
        let y = v[1];
        let angle = y.atan2(x);
        let perturb =  0.06*(30.0*angle).cos();
        4000.0*(4.0*(distance + perturb)*PI).cos()
    };

    let events = variable_poisson(lambda, 4000.0, &domain);

    // not printing this, the stdout gets flooded with all these evts
    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data
    let s = Scatter::from_slice(&data)
        .style(scatter::Style::new()
            .marker(Marker::Circle)
            .colour(MARKER_COLOR)
            .size(1.2)); // and a custom colour 
    
    // The 'view' describes what set of data is drawn
    let v = view::ContinuousView::new()
        .add(&s)
        .x_range(-1., 1.)
        .y_range(-1., 1.)
        .x_label("x")
        .y_label("y");
    
    page::Page::single(&v)
        .dimensions(600, 600)
        .save("examples/images/2d_poisson.variable.circle.svg").unwrap();
}
