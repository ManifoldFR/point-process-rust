use std::fs;

use plotlib::style::{
    PointStyle, PointMarker, 
    LineStyle
    };
use plotlib::page::Page;
use plotlib::view;
use plotlib::repr::{Line, Scatter};

use pointprocesses::hawkes_exponential;
use pointprocesses::TimeProcessResult;


static IMAGES_DIR: &str = "lib/examples/images";

fn main() {
    fs::create_dir(IMAGES_DIR).unwrap_or_default();
    println!("Fixed size jumps");
    fixed_jump();
}

fn fixed_jump() {
    
    let tmax = 90.0;
    let alpha = 0.2;
    let beta = 1.0;
    let lambda0 = 0.6;

    let events: TimeProcessResult = hawkes_exponential(tmax, beta, lambda0, alpha);
    let timestamps = &events.timestamps;
    let intensities = &events.intensities;
    let n_events: usize = timestamps.len();

    println!("{:?}", events);
    
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

    let samples = 3000;
    let times = (0..samples).map(|i| {
        i as f64*tmax/samples as f64
    });
    let intens_data: Vec<(f64,f64)> = times.into_iter().map(|t| {
        let lam = intensity_func(&events, t);
        //intens_data.push((t-0.0001/samples as f64, lam-alpha));
        (t, lam)
    }).collect();

    let intens_plot = Line::new(intens_data)
        .style(LineStyle::new()
            .width(1.2)
            .colour("#0971B2")
        );
    
    let mut ev_tupl: Vec<(f64,f64)> = Vec::new();
    for i in 0..n_events {
        ev_tupl.push(
            (timestamps[i], intensities[i])
        );
    }
    
    let sc = Scatter::from_slice(&ev_tupl)
        .style(PointStyle::new()
            .size(1.2)
            .colour("#FF0000")
            .marker(PointMarker::Cross)
        );

    let v = view::ContinuousView::new()
        .x_label("Time t")
        .y_label("λ(t)")
        .add(intens_plot)
        .add(sc);

    let save_path = [IMAGES_DIR, "hawkes_exp.svg"].join("/");
    Page::single(&v)
        .dimensions(600, 300)
        .save(save_path).unwrap();
}
