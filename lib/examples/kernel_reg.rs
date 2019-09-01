use pointprocesses::estimators::nadarayawatson;
use rand::prelude::*;

use std::fs;

use plotters::prelude::*;
use ndarray::Array1;


static TITLE_FONT: &str = "Arial";
static IMG_SIZE: (u32, u32) = (640, 480);

fn main() {
    use nadarayawatson::*;
    use std::f64::consts::PI;


    let func = |x: &f64| {
        (2. * PI * x + 0.1).sin() + 1.5 * x + 1.0 * x * x
    };

    let x_arr = Array1::linspace(0., 1., 20);
    let ref mut rng = thread_rng();
    let normal = rand_distr::StandardNormal;
    let sigma = 0.4;
    
    let mut z_arr = x_arr.map(func);
    z_arr.mapv_inplace(|y| {
        let eps: f64 = normal.sample(rng);
        y + sigma * eps
    });

    let x_dense_arr = Array1::linspace(0., 1., 50);
    let y_arr = x_dense_arr.map(func);

    // Setup chart

    fs::create_dir("examples/images").unwrap_or_default();
    let root = BitMapBackend::new(
        "lib/examples/images/nadwat_estimator.png",
        IMG_SIZE).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let caption = "";
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, (TITLE_FONT, 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0.0..1.0, -0.5..3.0)
        .unwrap();


    chart.configure_mesh().draw().unwrap();

    let noisy_data = x_arr.iter().zip(z_arr.iter());
    let size: u32 = 2;

    chart.draw_series(
        noisy_data
            .map(|(x,y)| {
                Circle::new((*x, *y), size, RED.filled())
            })
    ).unwrap();

    let data_reference_func = x_dense_arr.iter().zip(y_arr.iter()).map(
        |(x,y)| {
            (*x, *y)
        }
    );
    let line = LineSeries::new(data_reference_func, &BLUE);
    chart.draw_series(line)
        .unwrap()
        .label("Reference function")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Create and fit the NW estimator
    
    let bandwidth = 0.05;
    let kernel = GaussianKernel::new(bandwidth);
    let estimator = NadWatEstimator::new(kernel).fit(&x_arr, &z_arr);

    let x0_predict_points = x_dense_arr.clone();
    let y0_predict = x0_predict_points.map(
        |x0| estimator.predict(*x0)
    );

    let predict_data = x0_predict_points.iter().zip(y0_predict.iter())
        .map(|(x,y)| (*x, *y)
    );
    let line_predict = LineSeries::new(
        predict_data, &BLACK
    );
    chart.draw_series(line_predict)
        .unwrap()
        .label("NW estimates.")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLACK));
    

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().unwrap();
}

