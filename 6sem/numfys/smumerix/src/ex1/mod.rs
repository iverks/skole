use pyo3::{prelude::*, types::PyType};

mod edg;

#[pyclass(name = "EventDrivenGas", unsendable)]
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

    #[classmethod]
    fn new_uniform_v(_cls: &PyType) -> PyResult<Self> {
        Ok(Self {
            lib_edg: edg::EventDrivenGas::new_uniform_v(),
        })
    }

    fn step(&mut self) {
        self.lib_edg.step();
    }

    fn main(&self) -> PyResult<()> {
        println!("Hello from ex1");
        Ok(())
    }
}
