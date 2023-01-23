use ex0::level_crossing_prob_sim;
use ex0::start_point_sim;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod ex0;
mod ex1;

/// A Rust based numerics library
#[pymodule]
fn smumerix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(preex))?;
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_class::<ex1::PyEventDrivenGas>()?;
    Ok(())
}

/// Functions needed for pre exercise
#[pymodule]
fn preex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(one_a, m)?)?;
    m.add_function(wrap_pyfunction!(one_b, m)?)?;
    Ok(())
}

#[pyfunction]
fn main() -> PyResult<()> {
    println!("Hello world from rust");
    Ok(())
}

#[pyfunction]
fn one_a(num_loops: usize) -> PyResult<Vec<f64>> {
    let sim_res = start_point_sim(num_loops);
    Ok(ex0::probability_distribution(&sim_res))
}

#[pyfunction]
fn one_b(point: f64, num_loops: usize) -> PyResult<Vec<f64>> {
    let sim_res = level_crossing_prob_sim(point, num_loops);
    Ok(ex0::probability_distribution(&sim_res))
}
