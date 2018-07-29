#![feature(extern_prelude, use_extern_macros, specialization)]

extern crate pointprocesses;
extern crate pyo3;

use pointprocesses::*;

use pyo3::prelude::*;

struct EventWrapper(event::Event);

impl ToPyObject for EventWrapper {
    fn to_object(&self, py: Python) -> PyObject {
        let res = PyDict::new(py);
        res.set_item("timestamp", self.0.get_timestamp()).unwrap();
        res.set_item("intensity", self.0.get_intensity()).unwrap();
        res.into()
    }
}

impl IntoPyObject for EventWrapper {
    fn into_object(self, py: Python) -> PyObject {
        self.to_object(py)
    }
}

/// The point process library.
#[pymodinit]
fn pointprocesses(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "poisson_process")]
    fn poisson_process_py(_py: Python, tmax: f64, lambda: f64) -> Vec<EventWrapper> {
        let elements = poisson_process(tmax, lambda);
        elements.into_iter().map(|ev| EventWrapper(ev)).collect()
    }

    // TODO
    /*
    #[pyfn(m, "variable_poisson")]
    fn variable_poisson_py<F>(
        _py: Python, tmax: f64, lambda: F,
        max_lambda: f64) -> Vec<EventWrapper>
    where F: Fn(f64) -> f64
    {
        let elements = variable_poisson(tmax, lambda, max_lambda);
        elements.into_iter().map(|ev| EventWrapper(ev)).collect()
    }
    */

    Ok(())
}