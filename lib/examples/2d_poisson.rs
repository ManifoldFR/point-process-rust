use std::fs;

use ndarray::{array, s};
use ndarray::{Array1, Array2};

use pointprocesses::spatial::*;

use plotters::prelude::*;

static IMG_SIZE: (u32, u32) = (480, 480);
static TITLE_FONT: &str = "Arial";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir("lib/examples/images").unwrap_or_default();
    square_example()?;
    variable_circle_example()?;
    Ok(())
}

/// The `plotlib` library expects vectors of tuples for input.
fn cast_to_tuples(arr: Array2<f64>) -> Vec<(f64,f64)> {
    let n = arr.shape()[0];
    (0..n).map(|i: usize| {
        let v = arr.slice(s![i,..]);
        (v[0], v[1])
    }).collect()
}

fn square_example() -> Result<(), Box<dyn std::error::Error>> {
    let lambda = 200.0;

    let close = array![0.0,0.0];
    let far = array![1.0,1.0];
    let domain = Domain::new(close, far);

    let events = poisson_process(lambda, &domain);

    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let d = events.shape()[1];
    assert_eq!(d, 2);

    let data = cast_to_tuples(events);

    let root = BitMapBackend::new(
        "lib/examples/images/2d_poisson_rect.png", IMG_SIZE)
        .into_drawing_area();

    root.fill(&WHITE)?;

    let chart_title = "2D Poisson process";
    let mut chart = ChartBuilder::on(&root)
        .caption(chart_title, (TITLE_FONT, 16).into_font())
        .margin(5)  
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..1.0, 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    let size = 2;
    chart.draw_series(
        data.iter()
            .map(|(x,y)| {
                Circle::new((*x, *y), size,
                RED.filled())
            })   
    )?;

    Ok(())
}

fn variable_circle_example() -> Result<(), Box<dyn std::error::Error>> {
    let close: Array1<f64> = array![0.,0.];
    let far = array![1.,1.];
    let domain = Domain::new(close, far);

    // the intensity function will be the distance to center
    let lambda = |v: &Array1<f64>| {
        use std::f64::consts::PI;
        let center = array![0.5,0.5];
        let radius = 1.;
        let v = v - &center;
        let distance = v.dot(&v).sqrt();
        let x = v[0];
        let y = v[1];
        let angle = y.atan2(x);
        let perturb =  0.04*(10.0*angle).cos();
        if distance <= radius {
            4000.0 * (6.0 * (distance + perturb) * PI).cos()
        } else {
            0.0
        }
    };

    let events = variable_poisson(lambda, 4000.0, &domain);

    // not printing this, the stdout gets flooded with all these evts
    // println!("{:?}", events);
    println!("Simulated {} events.", events.shape()[0]);

    let data = cast_to_tuples(events);

    let root = BitMapBackend::new(
        "lib/examples/images/2d_poisson_circle.png", IMG_SIZE)
        .into_drawing_area();

    root.fill(&WHITE)?;

    let chart_title = "2D Poisson process";
    let mut chart = ChartBuilder::on(&root)
        .caption(chart_title, (TITLE_FONT, 16).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..1.0, 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    let size = 2;
    chart.draw_series(
        data.iter()
            .map(|(x,y)| {
                Circle::new((*x, *y), size,
                RED.filled())
            })   
    )?;

    Ok(())

}
