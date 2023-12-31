use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
mod dist_functions;
use dist_functions::levenshtein_distance::compute_levenshtein_distance;
use dist_functions::euclidean_distance::compute_euclidean_distance;

#[pymodule]
fn distance_functions(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_wrapped(wrap_pyfunction!(compute_levenshtein_distance));
    let _ = m.add_wrapped(wrap_pyfunction!(compute_euclidean_distance));
    Ok(())
}