use pointprocesses::{variable_poisson};
use pointprocesses::estimators::nadarayawatson;

use std::fs;

use plotters::prelude::*;

static IMG_SIZE: (u32, u32) = (720, 360);
static TITLE_FONT: &str = "Arial";


fn main() {
    println!("Oscillating intensity:");
    oscillating();
    println!("Exp-polynomial");
    polynom_exp();
}

fn oscillating() {
    let tmax = 30.0;
    
    let f: fn(f64) -> f64 = |t| {
        1.0*(30.0 + 0.95*t).ln()*(1.0 + 0.4*(0.5*t).sin())
    };

    let max_lbda = 6.0;
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

    let size = 2;
    chart
        .draw_series(
            timestamps.iter()
            .zip(intensities.iter())
            .map(|(x,y)| Circle::new((*x, *y), size, &RED))
        ).unwrap();

}

fn polynom_exp() {
    let tmax = 10.0;
    
    let f: fn(f64) -> f64 = |t| {
        2. * (3.0 * t * t + 1.2 * t + 9.) * (-0.5 * t).exp()
    };

    let max_lbda = 25.0;
    let events_tup = variable_poisson(tmax, &f, max_lbda);
    let timestamps = events_tup.timestamps;
    let intensities = events_tup.intensities;

    fs::create_dir("examples/images").unwrap_or_default();
    let root = BitMapBackend::new(
        "lib/examples/images/poisson_poly_exp.png",
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
    let time_arr: Vec<f64> = (0..101)
        .map(|i| dt * i as f64).collect();


    let intensity_t: Vec<_> = time_arr.iter()
        .map(|t| (*t, f(*t))).collect();

    let series = LineSeries::new(
        intensity_t, &RED);

    chart
        .draw_series(series).unwrap();

    let size = 2;
    chart
        .draw_series(
            timestamps.iter()
            .zip(intensities.iter())
            .map(|(x,y)| Circle::new((*x, *y), size, &RED))
        ).unwrap()
        .label("True intensity")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &RED));


    // PREDICT INTENSITY FROM EVENT SEQUENCES USING KERNEL SMOOTHER

    use pointprocesses::temporal::VariablePoissonProcess;
    use pointprocesses::temporal::TemporalProcess;

    // Get a bunch of event sequence samples
    let model = VariablePoissonProcess::new(&f, max_lbda);

    let num_samples = 80usize;
    let event_sequences: Vec<_> = model
        .batch_sample(tmax, num_samples)
        .into_iter()
        .map(|e| {
            e.timestamps
        }).collect();

    use nadarayawatson::UniformKernelIntensity;

    let bandwidth = 0.4;
    // Define and fit
    let estimator = UniformKernelIntensity::new(bandwidth)
        .fit(event_sequences);

    // Intensity predicted using the Kernel smoother
    let predicted_intens: Vec<_> = time_arr.iter().map(|t0| {
        (*t0, estimator.predict(*t0, tmax))
    }).collect();

    let pred_series = LineSeries::new(
        predicted_intens, &BLUE
    );

    chart.draw_series(pred_series).unwrap()
        .label("Estimated intensity")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();

}

