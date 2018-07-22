extern crate pointprocesses;
extern crate plotlib;
extern crate serde_json;

use plotlib::function;
use plotlib::scatter;
use plotlib::style::{Line, Marker, Point};
use plotlib::view;
use plotlib::page::Page;

use pointprocesses::event::Event;
use pointprocesses::variable_poisson;

use std::fs;

fn main() {
    
    let tmax = 30.0;
    let f: fn(f64) -> f64 = |t| {
        4.0*(30.0 - 0.95*t).ln()*(1.0 + 0.1*(0.5*t).sin())
    };
    let events: Vec<Event> = variable_poisson(tmax, f, 17.0);

    println!("{}", serde_json::to_string_pretty(&events).unwrap());
    
    let intens_plot = function::Function::new(f, 0.0, tmax)
        .style(function::Style::new().colour("#FF720C").width(1.5));
    
    let event_data: Vec<(f64,f64)> = events.into_iter()
        .map(|e: Event| (e.get_timestamp(), e.get_intensity()))
        .collect();

    let s = scatter::Scatter::from_slice(&event_data)
        .style(scatter::Style::new()
            .size(2.5)
            .marker(Marker::Cross)
            .colour("#1932E8"));
    
    let v = view::ContinuousView::new()
        .add(&s)
        .add(&intens_plot)
        .x_label("Time t")
        .y_label("Intensity λ(t)");
    
    fs::create_dir("examples/images").unwrap_or_default();
    Page::single(&v)
        .save("examples/images/variable_poisson.svg");
    
}
