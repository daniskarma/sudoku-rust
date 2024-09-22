use std::collections::HashSet;

/// Check that all elements in a vector are unique.
pub fn all_unique<T: std::hash::Hash + std::cmp::Eq>(vec: &[T]) -> bool {
    let mut set = HashSet::new();
    vec.iter().all(move |x| set.insert(x))
}

/// Generates list of combined numbers of n size that will be possible candidates
pub fn generate_combinations(n: usize) -> Vec<Vec<u8>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    backtrack(&mut results, &mut current, n, 1);
    results
}
// Backtrack function for generate_combinations()
fn backtrack(results: &mut Vec<Vec<u8>>, current: &mut Vec<u8>, n: usize, start: u8) {
    if current.len() == n {
        results.push(current.clone());
        return;
    }

    for i in start..=9 {
        current.push(i);
        backtrack(results, current, n, i + 1);
        current.pop();
    }
}
