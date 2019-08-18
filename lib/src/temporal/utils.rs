use rand::thread_rng;
use rand_distr::Distribution;
use rand_distr::DistIter;
use rand_distr::StandardNormal;


/// Simulate a brownian motion with a time step of h.
pub fn simulate_brownian(h: f64, n: usize) -> Vec<f64> {
    let sqr_h = h.sqrt();
    let ref mut rng = thread_rng();
    let normal: StandardNormal = StandardNormal;
    let mut normal_its: DistIter<_,_,f64> = normal.sample_iter(rng);

    let dwt: Vec<_> = (0..n-1).into_iter().map(|_| {
        sqr_h * normal_its.next().unwrap()
    }).collect();
    
    let mut wt: Vec<_> = vec![0.; n];
    wt[0] = 0.;
    for i in 0..n-1 {
        wt[i+1] = wt[i] + dwt[i];
    }
    wt
}

