use std::collections::HashSet;
use core::hash::Hash;
use pyo3::prelude::pyfunction;

#[pyfunction]
fn jaccard_similarity<T>(s1: Vec<T>, s2: Vec<T>) -> f32
where
    T: Hash + Eq + Clone,
{
    let s1 = vec_to_set(&s1);
    let s2 = vec_to_set(&s2);
    let i = s1.intersection(&s2).count() as f32;
    let u = s1.union(&s2).count() as f32;
    return i / u;
}

fn vec_to_set<T>(vec: &Vec<T>) -> HashSet<T>
where
    T: Hash + Eq + Clone,{
    HashSet::from_iter(vec.iter().cloned())
}


#[test]
fn test_jaccard_similarity() {
    let left = vec!["kitten", "sitting", "saturday", "sunday"];
    let right = vec!["kitten", "sitting", "saturday", "sunday"];
    assert_eq!(jaccard_similarity(left, right), 1.0);
    let left = vec![1,2,3,4];
    let right = vec![1,2,3,4];
    assert_eq!(jaccard_similarity(left, right), 1.0);
    let left = vec![1,2,3,4];
    let right = vec![2,2,3,4];
    assert_eq!(jaccard_similarity(left, right), 0.75);

}