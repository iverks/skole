use pyo3::prelude::*;

mod edg;

#[pyclass(name = "EventDrivenGas")]
pub struct PyEventDrivenGas {
    lib_edg: edg::EventDrivenGas,
}

#[pymethods]
impl PyEventDrivenGas {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            lib_edg: edg::EventDrivenGas::new(),
        })
    }

    fn main(&self) -> PyResult<()> {
        println!("Hello from ex1");
        Ok(())
    }
}
