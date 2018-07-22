extern crate pointprocesses;
extern crate plotlib;
extern crate svg;
extern crate serde_json;
extern crate rand;

use std::fs;
use std::path::Path;

use svg::Node;

use plotlib::style::{Point, Marker, Line};
use plotlib::view;
use plotlib::view::View;
use plotlib::line;
use plotlib::page;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use pointprocesses::event::Event;
use pointprocesses::hawkes_exponential;

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

    let events: Vec<Event> = hawkes_exponential(tmax, beta, lambda0, &mut jumps);

    println!("{}", serde_json::to_string_pretty(&events).unwrap());
    
    // Kernel function. Only used for plotting.
    let kernel = |t: f64| {
        if t >= 0.0 {
            alpha*(-beta*t).exp()
        } else {
            0.0
        }
    };

    let intensity_func = |events: &[Event], t: f64| {
        let result: f64 = events
            .iter()
            .take_while(|&ev| ev.get_timestamp() < t)
            .fold(0.0, |acc, ev| {
            acc+kernel(t - ev.get_timestamp())
        });
        result + lambda0
    };

    let samples = 3000;
    let times = (0..samples).map(|i| {
        i as f64*tmax/samples as f64
    });
    let mut intens_data: Vec<(f64,f64)> = Vec::new();

    for t in times {
        let lam = intensity_func(&events, t);
        //intens_data.push((t-0.0001/samples as f64, lam-alpha));
        intens_data.push((t, lam));
    };

    let intens_plot = line::Line::new(&intens_data)
        .style(line::Style::new()
            .width(1.2)
            .colour("#0971B2")
        );
    
    let ev_tupl: Vec<(f64,f64)> = events.into_iter()
        .map(|e: Event| {
            (e.get_timestamp(), e.get_intensity())
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
        .save("examples/images/hawkes_exp.fixed.svg");
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

    let events: Vec<Event> = hawkes_exponential(tmax, beta, lambda0, jumps);

    println!("{}", serde_json::to_string_pretty(&events).unwrap());
    
    
    // Kernel function. Only used for plotting.
    let kernel = |y: f64, t: f64| {
        if t >= 0.0 {
            y*(-beta*t).exp()
        } else {
            0.0
        }
    };

    let intensity_func = |events: &[Event], t: f64| {
        let result: f64 = events
            .iter()
            .take_while(|&ev| ev.get_timestamp() < t)
            .fold(0.0, |acc, ev| {
            acc+kernel(ev.get_jump(), t - ev.get_timestamp())
        });
        result + lambda0
    };

    let samples = 3000;
    let times = (0..samples).map(|i| {
        i as f64*tmax/samples as f64
    });
    let mut intens_data: Vec<(f64,f64)> = Vec::new();

    for t in times {
        let lam = intensity_func(&events, t);
        //intens_data.push((t-0.0001/samples as f64, lam-alpha));
        intens_data.push((t, lam));
    };
    
    let intens_plot = line::Line::new(&intens_data)
        .style(line::Style::new()
            .width(1.2)
            .colour("#0971B2")
        );
    
    let ev_tupl: Vec<(f64,f64)> = events.into_iter()
        .map(|e: Event| {
            (e.get_timestamp(), e.get_intensity())
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
        .save("examples/images/hawkes_exp.gamma_dist.svg");
}
