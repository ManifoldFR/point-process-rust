use pointprocesses::{variable_poisson};

use std::fs;

use plotters::prelude::*;

static IMG_SIZE: (u32, u32) = (720, 360);
static TITLE_FONT: &str = "Arial";


fn main() {
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

    let max_lbda = 17.0;
    let events_tup = variable_poisson(tmax, &f, max_lbda);
    let timestamps = events_tup.timestamps;
    let intensities = events_tup.intensities;

    fs::create_dir("examples/images").unwrap_or_default();
    let root = BitMapBackend::new(
        "lib/examples/images/poisson_oscillating.png",
        IMG_SIZE).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let caption = "Poisson process intensity";
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, (TITLE_FONT, 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..tmax, 0.0..max_lbda).unwrap();


    chart.configure_mesh().draw().unwrap();


    let dt = tmax / 100.;
    let time_arr = (0..101)
        .map(|i| dt * i as f64);


    let intensity_t = time_arr
        .map(|t| (t, f(t)));

    let series = LineSeries::new(
        intensity_t, &RED);

    chart
        .draw_series(series).unwrap();

}

fn decrease_exp() {
    let tmax = 30.0;
    
    let f: fn(f64) -> f64 = |t| {
        (-0.6 * t).exp() * 5.0
    };

    let max_lbda = 5.0;
    let events_tup = variable_poisson(tmax, &f, max_lbda);
    let timestamps = events_tup.timestamps;
    let intens = events_tup.intensities;

    fs::create_dir("examples/images").unwrap_or_default();
    let root = BitMapBackend::new(
        "lib/examples/images/poisson_exponential.png",
        IMG_SIZE).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let caption = "Poisson process intensity";
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, (TITLE_FONT, 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..tmax, 0.0..max_lbda).unwrap();


    chart.configure_mesh().draw().unwrap();

    let dt = tmax / 100.;
    let time_arr = (0..101)
        .map(|i| dt * i as f64);

    let intensity_t = time_arr
        .map(|t| (t, f(t)));

    let series = LineSeries::new(
        intensity_t, &RED);

    chart
        .draw_series(series).unwrap();

}

