extern crate pointprocesses;
extern crate plotlib;
extern crate serde_json;

use plotlib::function;
use plotlib::scatter;
use plotlib::style::{Line, Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;

use pointprocesses::event::Event;
use pointprocesses::variable_poisson;

fn main() {
    
    let tmax = 60.0;
    let f: fn(f64) -> f64 = |t| {
        1.0 + (0.5*t).sin()*(-0.05*t).exp()
    };
    let events: Vec<Event> = variable_poisson(tmax, f, 2.0);

    println!("{}", serde_json::to_string_pretty(&events).unwrap());
    
    let intens_plot = function::Function::new(f, 0.0, tmax)
        .style(function::Style::new().colour("#4C36EB").width(1.5));
    
    let event_data: Vec<(f64,f64)> = events.into_iter()
        .map(|e: Event| (e.timestamp(), e.intensity()))
        .collect();

    let s = scatter::Scatter::from_vec(&event_data)
        .style(scatter::Style::new()
            .size(2.5)
            .marker(Marker::Cross)
            .colour("#E0A536"));
    
    let v = View::new()
        .add(&s)
        .add(&intens_plot)
        .x_label("Temps t")
        .y_label("Intensité λ(t)");
    
    Page::single(&v).save("examples/variable_poisson.svg");
    
}
