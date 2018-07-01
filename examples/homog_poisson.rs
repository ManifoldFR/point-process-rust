extern crate pointprocesses;
extern crate gnuplot;
extern crate serde_json;

use gnuplot::{Figure,Caption,Color,PointSymbol,PointSize};

use pointprocesses::event::Event;
use pointprocesses::poisson_process;


fn main() {
    
    let tmax = 10.0;
    let lambda = 3.0;

    let events: Vec<Event> = poisson_process(tmax, lambda);

    println!("{}",
        serde_json::to_string_pretty(&events).unwrap()
    );
    

    let mut event_times: Vec<f64> = vec!();
    let mut event_intens: Vec<f64> = vec!();
    for i in 0..events.len() {
        let event = &events[i];
        event_times.push(event.timestamp);
        event_intens.push(event.intensity());
    }

    let mut fg = Figure::new();
    
    fg.axes2d()
        .points(&event_times, &event_intens, 
            &[
                Caption("Événements"),
                Color("black"),
                PointSymbol('O'),
                PointSize(0.8)]);
    fg.show();
    fg.echo_to_file("test.gnuplot");


}
