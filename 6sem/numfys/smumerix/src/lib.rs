use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod ex0;

/// A Python module implemented in Rust.
#[pymodule]
fn smumerix(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(preex))?;
    Ok(())
}

#[pymodule]
fn preex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    m.add_function(wrap_pyfunction!(probability_distribution, m)?)?;
    Ok(())
}

#[pyfunction]
fn main() -> PyResult<()> {
    let dist = ex0::probability_distribution();
    println!("{dist:?}");
    Ok(())
}

#[pyfunction]
fn probability_distribution() -> PyResult<Vec<f64>> {
    Ok(ex0::probability_distribution())
}
