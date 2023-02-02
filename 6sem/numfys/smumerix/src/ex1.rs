use std::collections::BinaryHeap;

use nalgebra::{Point2, Vector2};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyType};
use smumerix_core::edg::{self, EventDrivenGas, Particle};

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

    #[classmethod]
    fn new_for_test_4(_cls: &PyType, y: f64) -> PyResult<Self> {
        if !(-0.4 < y && y < 0.4) {
            return Err(PyValueError::new_err("Y should be between -0.4 and 0.4"));
        }
        let pq = BinaryHeap::new();
        let particles: Vec<Particle> = vec![
            Particle {
                x: Point2::new(0.5, 0.5),
                v: Vector2::new(0.0, 0.0),
                r: 0.1,
                m: 1e6,
                collision_count: 0,
            },
            Particle {
                x: Point2::new(0.3, 0.5 + y),
                v: Vector2::new(0.2, 0.0),
                r: 0.001,
                m: 1.0,
                collision_count: 0,
            },
        ];

        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };
        edg.get_initial_collisions();

        Ok(Self { lib_edg: edg })
    }

    fn get_angle_off_x_axis(&self, particle_idx: usize) -> PyResult<f64> {
        let x_axis = Vector2::new(-1.0, 0.0);
        Ok(self.lib_edg.particles[particle_idx].v.angle(&x_axis))
    }

    fn step(&mut self) {
        self.lib_edg.step();
    }

    fn step_many(&mut self, num_steps: i32) {
        self.lib_edg.step_many(num_steps)
    }

    fn get_positions(&self) -> (Vec<f64>, Vec<f64>) {
        (
            self.lib_edg.particles.iter().map(|p| p.x.x).collect(),
            self.lib_edg.particles.iter().map(|p| p.x.y).collect(),
        )
    }

    fn get_sizes(&self) -> Vec<f64> {
        self.lib_edg.particles.iter().map(|p| p.r).collect()
    }

    fn get_total_energy(&self) -> f64 {
        self.lib_edg.get_total_energy()
    }

    fn main(&self) -> PyResult<()> {
        println!("Hello from ex1");
        Ok(())
    }
}
