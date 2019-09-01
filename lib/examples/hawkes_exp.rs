use std::fs;

use plotters::prelude::*;

use pointprocesses::hawkes_exponential;
use pointprocesses::temporal::hawkes;
use pointprocesses::TimeProcessResult;

static TITLE_FONT: &str = "Arial";
static IMAGES_DIR: &str = "lib/examples/images";

fn main() {
    fs::create_dir(IMAGES_DIR).unwrap_or_default();
    let img_size = (720, 360);
    println!("Exp Hawkes w/ constant background intensity.");
    const_background(img_size);
    println!("Exp Hawkes w/ variable background intensity.");
    variable_background(img_size);
}

fn const_background(img_size: (u32, u32)) {
    
    let tmax = 60.0;
    let alpha = 0.2;
    let beta = 0.8;
    let lambda0 = 0.6;

    let events: TimeProcessResult = hawkes_exponential(
        tmax, alpha, beta, lambda0);
    let timestamps = &events.timestamps;
    let intensities = &events.intensities;
    
    // Kernel function. Only used for plotting.
    let kernel = |t: f64| {
        if t >= 0.0 {
            alpha*(-beta*t).exp()
        } else {
            0.0
        }
    };

    let intensity_func = |events: &TimeProcessResult, t: f64| {
        let result: f64 = events.timestamps
            .into_iter()
            .take_while(|&ev| ev < &t)
            .fold(0.0, |acc, ev| {
            acc+kernel(t - ev)
        });
        result + lambda0
    };

    fs::create_dir("examples/images").unwrap_or_default();
    let root = BitMapBackend::new(
        "lib/examples/images/hawkes_exp_constbg.png",
        img_size).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let upper_lbda_bound = 3.0;

    let caption = "Hawkes process intensity.";
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, (TITLE_FONT, 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..tmax, 0.0..upper_lbda_bound)
        .unwrap();


    chart.configure_mesh().draw().unwrap();

    let num_plot_tsamples = 1001;
    let dt = tmax / (num_plot_tsamples - 1) as f64;
    let time_arr: Vec<f64> = (0..num_plot_tsamples)
        .map(|i| dt * i as f64).collect();


    let bg_intensity_t = time_arr
        .iter()
        .map(|t| (*t, lambda0));

    let intensity_t = time_arr
        .iter()
        .map(|t| (*t, intensity_func(&events, *t)));

    let series = LineSeries::new(
        intensity_t, &RED);

    let bg_series = LineSeries::new(
        bg_intensity_t, &BLACK);

    chart
        .draw_series(series).unwrap();

    chart
        .draw_series(bg_series).unwrap();
}

/// Example sampling from an exponential-kernel Hawkes process
/// with variable background intensity.
fn variable_background(img_size: (u32, u32)) {
    use hawkes::{Hawkes,DeterministicBackground, ExpKernel, Kernel};
    use pointprocesses::temporal::{TemporalProcess, DeterministicIntensity};

    let tmax = 60.0;
    let alpha = 0.2;
    let beta = 0.8;
    let max_lbda0 = 1.0;
    let omega = 0.3;  // frequency of the background signal

    let lbda0 = |t: f64| {
        1. + 0.5 * (omega * t).cos()
    };

    let model = Hawkes::<DeterministicBackground<_>, ExpKernel>::new(
        alpha, beta, lbda0, max_lbda0
    );

    let kernel = model.get_kernel();
    let background = model.get_background();
    
    let events: TimeProcessResult = model.sample(tmax);
    let timestamps = &events.timestamps;
    let intensities = &events.intensities;
    
    // Kernel function. Only used for plotting.
    let intensity_func = |events: &TimeProcessResult, t: f64| {
        let auto_intens: f64 = events.timestamps
            .into_iter()
            .take_while(|&ev| ev < &t)
            .fold(0.0, |acc, ev| {
            acc + kernel.eval(t - ev)
        });
        auto_intens + background.intensity(t)
    };
    
    fs::create_dir("examples/images").unwrap_or_default();
    let root = BitMapBackend::new(
        "lib/examples/images/hawkes_exp_sine_bg.png",
        img_size).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let upper_lbda_bound = 3.0;

    let caption = "Hawkes process intensity.";
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, (TITLE_FONT, 20).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..tmax, 0.0..upper_lbda_bound)
        .unwrap();


    chart.configure_mesh().draw().unwrap();

    let num_plot_tsamples = 1001;
    let dt = tmax / (num_plot_tsamples - 1) as f64;
    let time_arr: Vec<f64> = (0..num_plot_tsamples)
        .map(|i| dt * i as f64).collect();


    let bg_intensity_t = time_arr
        .iter()
        .map(|t| (*t, lbda0(*t)));

    let intensity_t = time_arr
        .iter()
        .map(|t| (*t, intensity_func(&events, *t)));

    let series = LineSeries::new(
        intensity_t, &RED);

    let bg_series = LineSeries::new(
        bg_intensity_t, &BLACK);

    chart
        .draw_series(series).unwrap();

    chart
        .draw_series(bg_series).unwrap();

}

