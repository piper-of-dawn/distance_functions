//! # Edit distance
//!
//! The [Levenshtein edit distance][wikipedia] between two strings is
//! the number of individual single-character changes (insert, delete,
//! substitute) necessary to change string `left` into `right`.
//!
//! This can be left used to order search results, for fuzzy
//! auto-completion, and to find candidates for spelling correction.
//!
//! ## References
//! [Wikipedia: Levenshtein distance][wikipedia]  
//! [NIST: Levenshtein distance][nist]
//!
//! [wikipedia]: http://en.wikipedia.org/wiki/Levenshtein_distance
//! [nist]: http://xlinux.nist.gov/dads/HTML/Levenshtein.html

/// Returns the edit distance between strings `left` and `right`.
///
/// The runtime complexity is `O(m*n)`, where `m` and `n` are the
/// strings' lengths.
///
/// # Examples
///
/// ```
/// use edit_distance::edit_distance;
///
/// edit_distance("kitten", "sitting"); // => 3
/// ```
/// 
use rayon::prelude::*;
use pyo3::prelude::pyfunction;

fn levenshtein_distance(left: &str, right: &str) -> usize {
    let len_left = left.chars().count();
    let len_right = right.chars().count();
    if len_left < len_right{
        return levenshtein_distance(right, left) // The length of left is always greater than or equal to right
    }
    // handle special case of 0 length
    if len_left == 0 {
        return len_right
    } else if len_right == 0 {
        return len_left
    }

    let len_right = len_right + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_right];

    // initialize string right
    for i in 1..len_right {
        cur[i] = i;
    }
    // calculate edit distance
    for (i,ca) in left.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in right.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1, std::cmp::min(
                // insertion
                cur[j] + 1,
                // match or substitution
                pre + if ca == cb { 0 } else { 1 }));
            pre = tmp;
        }
    }
    cur[len_right - 1]
}

#[pyfunction]
pub fn compute_levenshtein_distance(left: Vec<&str>, right: Vec<&str>) -> Vec<usize> {
    // This is a parallel version of the levenshtein distance.
    let zipped_series: Vec<(&str, &str)> = left.into_iter().zip(right.into_iter()).collect();
    let differenced = zipped_series
        .par_iter()
        .map(|(a, b)| levenshtein_distance(&a, &b))
        .collect::<Vec<usize>>();
    differenced
}

// pub fn compute_levenshtein_distance_non_parallel(left: &Vec<&str>, right: &Vec<&str>) -> Vec<usize> {
//     let zipped_series: Vec<(&&str, &&str)> = left.into_iter().zip(right.into_iter()).collect();
//     let differenced = zipped_series
//         .iter()
//         .map(|(&a, &b)| levenshtein_distance(&a, &b))
//         .collect::<Vec<usize>>();
//     differenced
// }

    #[test]
    fn test_levenshtein_distance () {
        let left = vec!["kitten", "sitting"];
        let right = vec!["sitting", "kitten"];
        let result = compute_levenshtein_distance(left, right);
        assert_eq!(result, vec![3, 3]);
    }
