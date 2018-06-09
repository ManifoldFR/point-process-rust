extern crate pointprocesses;
extern crate plotlib;

use plotlib::style::{Line,Point};
use plotlib::page::Page;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use pointprocesses::event::Event;
use pointprocesses::poisson_process;



fn main() {
    
    let tmax = 10.0;
    let lambda = 3.0;

    let events: Vec<Event> = poisson_process(tmax, lambda);

    println!("{:#?}", events);
    
    let mut ty: Vec<(f64,f64)> = vec!();
    for i in 0..events.len() {
        ty.push((events[i].timestamp, 0.001*i as f64));
    }

    let ty_scatter = Scatter::from_vec(&ty)
        .style(scatter::Style::new()
            .colour("brown")
            .marker(plotlib::style::Marker::Circle)
            .size(2.0));
    
    let myview = plotlib::view::View::new()
        .add(&ty_scatter)
        .x_label("Temps t")
        .y_label("Intensité")
        .y_range(-0.1, 0.1);
    
    Page::single(&myview).save("test.svg");
}
