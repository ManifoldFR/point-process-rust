extern crate pointprocesses;
extern crate plotlib;

use plotlib::style::{Line,Point};
use plotlib::function::Function;
use plotlib::page::Page;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use pointprocesses::event::Event;
use pointprocesses::{poisson_process,variable_poisson};



fn main() {
    
    let tmax = 60.0;
    let f: fn(f64) -> f64 = |t| 1.0+(1.1+0.5*t.sin())*(-0.05*t).exp();
    let variable_events: Vec<Event> = variable_poisson(tmax, f, 1.5);

    println!("{:#?}", variable_events);

    let functionplot = Function::new(f, 0.0, tmax)
        .style(&plotlib::function::Style::new()
            .colour("burlywood").width(1.0)
        );
    
    let mut ty: Vec<(f64,f64)> = vec!();
    for i in 0..variable_events.len() {
        let event = &variable_events[i];
        ty.push((event.timestamp, event.intensity()));
    }

    let ty_scatter = Scatter::from_vec(&ty)
        .style(scatter::Style::new()
            .colour("brown")
            .marker(plotlib::style::Marker::Circle)
            .size(2.0));
    
    let myview = plotlib::view::View::new()
        .add(&functionplot)
        .add(&ty_scatter)
        .x_label("Temps t")
        .y_label("Intensité");
    
    Page::single(&myview).save("test.svg");
}
