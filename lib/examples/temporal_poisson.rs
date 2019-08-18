use pointprocesses::{poisson_process,variable_poisson};
use plotlib::repr::{Function, Scatter};
use std::fs;
use plotlib::view;
use plotlib::page::Page;
use plotlib::style::PointStyle;
use plotlib::style::PointMarker;
use plotlib::style::LineStyle;

static MARKER_COLOR: &str = "#9B2636";
static LINE_COLOR: &str = "#2B2A2B";


fn main() {
    let tmax = 10.0;
    let lambda = 3.0;
    let events = poisson_process(tmax, lambda);
    println!("{:?}", events);

    println!("Oscillating intensity:");
    oscillating();
    println!("Decreasing exponential intensity:");
    decrease_exp();
}

fn oscillating() {
    let tmax = 30.0;
    let f: fn(f64) -> f64 = |t| {
        4.0*(30.0 - 0.95*t).ln()*(1.0 + 0.1*(0.5*t).sin())
    };
    let events_tup = variable_poisson(tmax, &f, 17.0);
    let timestamps = events_tup.timestamps;
    let intensities = events_tup.intensities;

    println!("{:?}", timestamps);


    let event_data: Vec<(f64,f64)> = timestamps.into_iter()
        .zip(intensities.into_iter())
        .map(|(t, l)| (*t, *l))
        .collect();

    let intens_plot = Function::new(f, 0.0, tmax)
        .style(LineStyle::new().colour(LINE_COLOR).width(2.0));

    let s = Scatter::from_slice(&event_data)
        .style(PointStyle::new()
            .size(2.5)
            .marker(PointMarker::Circle)
            .colour(MARKER_COLOR));

    let v = view::ContinuousView::new()
        .add(s)
        .add(intens_plot)
        .x_label("Time t")
        .y_label("Intensity λ(t)");

    fs::create_dir("examples/images").unwrap_or_default();
    Page::single(&v)
        .save("lib/examples/images/oscillating_poisson.svg").unwrap();

}

/// This example will occasionally return an empty array, which might happen
/// for very low intensity (or negative intensity) processes.
/// The error when stacking an empty `ndarray::Array` slice is handled.
fn decrease_exp() {
    let tmax = 30.0;
    let f: fn(f64) -> f64 = |t| {
        (-0.6 * t).exp() * 5.0
    };
    let events_tup = variable_poisson(tmax, &f, 5.0);
    let timestamps = events_tup.timestamps;
    let intens = events_tup.intensities;

    println!("{:?}", timestamps);

    let event_data: Vec<(f64,f64)> = timestamps.into_iter()
        .zip(intens.into_iter())
        .map(|(t, l)| (*t, *l))
        .collect();

    let intens_plot = Function::new(f, 0.0, tmax)
        .style(LineStyle::new().colour(LINE_COLOR).width(2.0));

    let s = Scatter::from_slice(&event_data)
        .style(PointStyle::new()
            .size(2.5)
            .marker(PointMarker::Circle)
            .colour(MARKER_COLOR));

    let v = view::ContinuousView::new()
        .add(s)
        .add(intens_plot)
        .x_label("Time t")
        .y_label("Intensity λ(t)");

    fs::create_dir("examples/images").unwrap_or_default();
    Page::single(&v)
        .save("lib/examples/images/exponential_poisson.svg").unwrap();
}

