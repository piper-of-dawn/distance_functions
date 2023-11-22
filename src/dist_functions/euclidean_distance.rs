use pyo3::prelude::pyfunction;
use rayon::prelude::*;

#[pyfunction]
pub fn compute_euclidean_distance (left: Vec<f64>, right: Vec<f64>) -> f64 {
    // This is a parallel version of the euclidean distance.
    let zipped_series: Vec<(f64, f64)> = left.into_iter().zip(right.into_iter()).collect();
    let squared_differences = zipped_series
        .into_par_iter()
        .map(|(a, b)| (a - b) * (a - b))
        .collect::<Vec<f64>>();
    // Sum up the squared differences in parallel.
    let sum_squared_differences = squared_differences
        .into_par_iter()
        // .cloned()
        .reduce(|| f64::default(), |acc, x| acc + x);

    sum_squared_differences.sqrt()
}


#[test]
fn test_euclidean_distance () {
    let left: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    let right: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    let result = compute_euclidean_distance(left, right);
    let left: Vec<f64> = vec![1.0, 2.0, 7.0, 4.0];
    let right: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    assert_eq!(result, 0.0);
}