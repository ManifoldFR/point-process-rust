extern crate pointprocesses;
extern crate gnuplot;
extern crate serde_json;

use gnuplot::{Figure,Caption,Color,PointSymbol,PointSize};
use gnuplot::AxesCommon;

use pointprocesses::variable_poisson;

fn main() {
    
    let tmax = 60.0;
    let f: fn(f64) -> f64 = |t| {
        1.0 + (0.5*t).sin()*(-0.05*t).exp()
    };
    let events = variable_poisson(tmax, f, 2.0);

    println!("{}", serde_json::to_string_pretty(&events).unwrap());
    
    // Plotting
    let num_points = 100;
    let times: Vec<f64> = (0..num_points).map(|x| {
        tmax*x as f64/num_points as f64
    }).collect();

    let lambda_values: Vec<f64> = times.iter()
        .map(|&x| f(x)).collect();

    let mut event_times: Vec<f64> = vec!();
    let mut event_intens: Vec<f64> = vec!();
    for i in 0..events.len() {
        let event = &events[i];
        event_times.push(event.timestamp);
        event_intens.push(event.intensity());
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
