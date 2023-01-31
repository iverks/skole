use pyo3::{exceptions::PyValueError, prelude::*, types::PyType};
use smumerix_core::edg;

#[pyclass(name = "EventDrivenGas", unsendable)]
pub struct PyEventDrivenGas {
    lib_edg: edg::EventDrivenGas,
}

#[pymethods]
impl PyEventDrivenGas {
    #[new]
    fn new() -> PyResult<Self> {
        let lib_edg = edg::EventDrivenGas::new();
        match lib_edg {
            Ok(lib_edg) => Ok(Self { lib_edg }),
            Err(_) => Err(PyValueError::new_err("Couldn't generate system")),
        }
    }

    #[classmethod]
    fn new_uniform_v(_cls: &PyType, num_particles: i32, speed: f64, radius: f64) -> PyResult<Self> {
        let lib_edg = edg::EventDrivenGas::new_uniform_v(num_particles, speed, radius);
        match lib_edg {
            Ok(lib_edg) => Ok(Self { lib_edg }),
            Err(_) => Err(PyValueError::new_err("Couldn't generate system")),
        }
    }

    fn step(&mut self) {
        self.lib_edg.step();
    }

    fn step_many(&mut self, num_steps: i32) {
        self.lib_edg.step_many(num_steps)
    }

    fn get_positions(&self) -> (Vec<f64>, Vec<f64>) {
        (
            self.lib_edg
                .particles
                .iter()
                .map(|p| p.borrow().x.x)
                .collect(),
            self.lib_edg
                .particles
                .iter()
                .map(|p| p.borrow().x.y)
                .collect(),
        )
    }

    fn get_sizes(&self) -> Vec<f64> {
        self.lib_edg
            .particles
            .iter()
            .map(|p| p.borrow().r)
            .collect()
    }

    fn get_total_energy(&self) -> f64 {
        self.lib_edg.get_total_energy()
    }

    fn main(&self) -> PyResult<()> {
        println!("Hello from ex1");
        Ok(())
    }
}
