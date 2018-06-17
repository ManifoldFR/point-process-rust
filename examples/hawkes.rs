extern crate pointprocesses;
extern crate gnuplot;

use gnuplot::{Figure,Caption,Color,PointSymbol,PointSize};
use gnuplot::AxesCommon;

use pointprocesses::event::Event;
use pointprocesses::hawkes_exponential;

fn main() {
    
    let tmax = 20.0;
    let alpha = 0.6;
    let beta = 0.8;
    let lambda0 = 1.0;

    let events = hawkes_exponential(tmax, alpha, beta, lambda0);

    println!("{:#?}", events);
    
    // Plotting
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
            .take_while(|ev| ev.timestamp < t)
            .map(|ev| {
            kernel(t - ev.timestamp)
        }).sum();
        result + lambda0
    };

    let num_points = 100;
    let times: Vec<f64> = (0..num_points).map(|i| {
        tmax*i as f64/num_points as f64
    }).collect();

    let lambda_values: Vec<f64> = times
        .iter()
        .map(|&t| intensity_func(&events, t))
        .collect();

    let mut event_times: Vec<f64> = vec![];
    let mut event_intens: Vec<f64> = vec![];
    for i in 0..events.len() {
        let event = &events[i];
        event_times.push(event.timestamp);
        event_intens.push(event.intensity().unwrap());
    }

    let mut fg = Figure::new();

    fg.axes2d()
        .lines(&times, &lambda_values,
            &[
                Caption("λ(t)"),
                Color("#1E90FF"),
            ])
        .points(&event_times, &event_intens, 
            &[
                Caption("Events"),
                Color("black"),
                PointSymbol('O'),
                PointSize(0.8)])
        .set_x_label("Temps t", &[])
        .set_y_label("Intensité", &[]);
    fg.echo_to_file("test.gnuplot");
    fg.show();
}
