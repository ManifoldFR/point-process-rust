extern crate pointprocesses;
extern crate plotlib;
extern crate svg;

#[macro_use]
extern crate ndarray;

use svg::Node;

use std::path::Path;

use plotlib::style::{Point, Marker};
use plotlib::view;
use plotlib::view::View;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use ndarray::{Array, Ix2};

use pointprocesses::generalized::*;

fn main() {
    square_example();
    circle_example();
}

/// The `plotlib` library expects vectors of tuples for input.
fn cast_to_tuples(arr: Array<f64,Ix2>) -> Vec<(f64,f64)> {
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
    println!("Bounding box: {:?}", domain.bounding_box());
    println!("Simulated {} events.", events.shape()[0]);

    let d = events.shape()[1];
    assert_eq!(d, 2);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data$
    let s1 = Scatter::from_slice(&data)
        .style(scatter::Style::new()
            .marker(Marker::Circle) // setting the marker to be a square
            .colour("#DD3355")
            .size(2.5)); // and a custom colour 
    
    let v = view::ContinuousView::new()
        .add(&s1)
        .x_range(0., 1.)
        .y_range(0., 1.)
        .x_label("x")
        .y_label("y");

    let mut document = svg::Document::new().set("viewBox", (0,0,600,600));
    let savepath = Path::new("examples/images/2d_poisson.rect.svg");
    document.append(v.to_svg(520., 520.).set("transform", format!("translate({}, {})", 50, 550)));
    svg::save(savepath, &document).unwrap();
}


fn circle_example() {
    let lambda = 400.0;

    let center = array![1.0,1.0];
    let radius = 1.0;
    let domain = Ball::new(center, radius);

    let events = poisson_process(lambda, &domain);

    // not printing this, the stdout gets flooded with all these evts
    // println!("{:?}", events);
    println!("Bounding box: {:?}", domain.bounding_box());
    println!("Simulated {} events.", events.shape()[0]);

    let data = cast_to_tuples(events);

    // We create our scatter plot from the data
    let s = Scatter::from_slice(&data)
        .style(scatter::Style::new()
            .marker(Marker::Circle) // setting the marker to be a square
            .colour("#0971B2")
            .size(2.0)); // and a custom colour 
    
    // The 'view' describes what set of data is drawn
    let v = view::ContinuousView::new()
        .add(&s)
        .x_range(0., 2.)
        .y_range(0., 2.)
        .x_label("x")
        .y_label("y");

    let mut document = svg::Document::new().set("viewBox", (0,0,600,600));
    let savepath = Path::new("examples/images/2d_poisson.circle.svg");
    document.append(v.to_svg(520., 520.).set("transform", format!("translate({}, {})", 50, 550)));
    svg::save(savepath, &document).unwrap();
    
}
