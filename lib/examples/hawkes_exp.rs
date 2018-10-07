extern crate pointprocesses;
extern crate plotlib;
extern crate svg;
extern crate rand;
extern crate ndarray;

use std::fs;

use plotlib::style::{Point, Marker, Line};
use plotlib::view;
use plotlib::line;
use plotlib::page;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use pointprocesses::hawkes_exponential;

use ndarray::prelude::*;

fn main() {
    fs::create_dir("examples/images").unwrap_or_default();
    fixed_jump();
    random_jumps();
}

fn fixed_jump() {
    
    let tmax = 90.0;
    let alpha = 0.2;
    let mut jumps = std::iter::repeat(alpha);
    let beta = 1.0;
    let lambda0 = 0.6;

    let events: Array2<f64> = hawkes_exponential(tmax, beta, lambda0, &mut jumps);

    println!("{:?}", events);
    
    // Kernel function. Only used for plotting.
    let kernel = |t: f64| {
        if t >= 0.0 {
            alpha*(-beta*t).exp()
        } else {
            0.0
        }
    };

    let intensity_func = |events: ArrayView2<f64>, t: f64| {
        let result: f64 = events
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
        let lam = intensity_func(events.view(), t);
        //intens_data.push((t-0.0001/samples as f64, lam-alpha));
        (t, lam)
    }).collect();

    let intens_plot = line::Line::new(&intens_data)
        .style(line::Style::new()
            .width(1.2)
            .colour("#0971B2")
        );
    
    let ev_tupl: Vec<(f64,f64)> = events.outer_iter()
        .map(|v| {
            (v[0], v[1])
        }).collect();
    
    let sc = Scatter::from_slice(&ev_tupl)
        .style(scatter::Style::new().size(1.2)
            .colour("#FF0000")
            .marker(Marker::Circle)
        );

    let v = view::ContinuousView::new()
        .x_label("Time t")
        .y_label("Intensity λ(t)")
        .add(&intens_plot)
        .add(&sc);

    page::Page::single(&v)
        .dimensions(900, 400)
        .save("examples/images/hawkes_exp.fixed.svg").unwrap();
}

fn random_jumps() {
    use rand::prelude::*;
    let ref mut rng = thread_rng();

    // Uses jumps distributed according to the the Gamma distributions
    let gamma = rand::distributions::Gamma::new(2.0, 2.0);
    let mut gamma_iter = gamma.sample_iter(rng);

    // get a mutable reference to the iterator
    let jumps = gamma_iter.by_ref();

    let tmax = 90.0;

    let beta = 1.0;
    let lambda0 = 0.9;

    let events: Array2<f64> = hawkes_exponential(tmax, beta, lambda0, jumps);

    println!("{:?}", events);
    
    
    // Kernel function. Only used for plotting.
    let kernel = |y: f64, t: f64| {
        if t >= 0.0 {
            y*(-beta*t).exp()
        } else {
            0.0
        }
    };

    let intensity_func = |events: ArrayView2<f64>, t: f64| {
        let result: f64 = events
            .outer_iter()
            .take_while(|&ev| ev[0] < t)
            .fold(0.0, |acc, ev| {
            acc+kernel(ev[2], t - ev[0])
        });
        result + lambda0
    };

    let samples = 3000;
    let times = (0..samples).map(|i| {
        i as f64*tmax/samples as f64
    });
    let mut intens_data: Vec<(f64,f64)> = Vec::new();

    for t in times {
        let lam = intensity_func(events.view(), t);
        //intens_data.push((t-0.0001/samples as f64, lam-alpha));
        intens_data.push((t, lam));
    };
    
    let intens_plot = line::Line::new(&intens_data)
        .style(line::Style::new()
            .width(1.2)
            .colour("#0971B2")
        );
    
    let ev_tupl: Vec<(f64,f64)> = events.outer_iter()
        .map(|ev| {
            (ev[0], ev[1])
        }).collect();
    
    let sc = Scatter::from_slice(&ev_tupl)
        .style(scatter::Style::new().size(1.2)
            .colour("#FF0000")
            .marker(Marker::Circle)
        );

    let v = view::ContinuousView::new()
        .x_label("Time t")
        .y_label("Intensity λ(t)")
        .add(&intens_plot)
        .add(&sc);

    page::Page::single(&v)
        .dimensions(900, 400)
        .save("examples/images/hawkes_exp.gamma_dist.svg").unwrap();
}
