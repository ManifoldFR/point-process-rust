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


#[pymodinit]
fn pointprocesses(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "poisson_process")]
    fn poisson_process_py(tmax: f64, lambda: f64) -> Vec<EventWrapper> {
        let elements = poisson_process(tmax, lambda);
        elements.into_iter().map(|ev| EventWrapper(ev)).collect()
    }

    Ok(())
}