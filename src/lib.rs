use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod distance_functions;
use distance_functions::levenshtein_distance::__pyo3_get_function_compute_levenshtein_distance;

#[pymodule]
fn distance_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_wrapped(wrap_pyfunction!(compute_levenshtein_distance));
    Ok(())
}