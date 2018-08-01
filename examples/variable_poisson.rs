extern crate pointprocesses;
extern crate plotlib;
extern crate ndarray;

use plotlib::function;
use plotlib::scatter;
use plotlib::style::{Line, Marker, Point};
use plotlib::view;
use plotlib::page::Page;

use pointprocesses::variable_poisson;

use ndarray::prelude::*;

use std::fs;

static LINE_COLOR: &str = "#373AC6";
static MARKER_COLOR: &str = "#1C58CE";

fn main() {
    
    let tmax = 30.0;
    let f: fn(f64) -> f64 = |t| {
        4.0*(30.0 - 0.95*t).ln()*(1.0 + 0.1*(0.5*t).sin())
    };
    let events = variable_poisson(tmax, f, 17.0);

    println!("{:?}", events);
    
    let intens_plot = function::Function::new(f, 0.0, tmax)
        .style(function::Style::new().colour(LINE_COLOR).width(1.5));
    
    let event_data: Vec<(f64,f64)> = events.axis_iter(Axis(0))
        .map(|ev| (ev[0], ev[1]))
        .collect();

    let s = scatter::Scatter::from_slice(&event_data)
        .style(scatter::Style::new()
            .size(2.5)
            .marker(Marker::Circle)
            .colour(MARKER_COLOR));
    
    let v = view::ContinuousView::new()
        .add(&s)
        .add(&intens_plot)
        .x_label("Time t")
        .y_label("Intensity λ(t)");
    
    fs::create_dir("examples/images").unwrap_or_default();
    Page::single(&v)
        .save("examples/images/variable_poisson.svg");
    
}
