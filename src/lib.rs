use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::prelude::pyfunction;
mod distance_functions;
use distance_functions::levenshtein_distance::__pyo3_get_function_compute_levenshtein_distance;


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}


#[pymodule]
fn distance_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(compute_levenshtein_distance));
    Ok(())
}