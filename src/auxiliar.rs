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
    backtrack(&mut results, &mut current, n, 1);
    results
}

#[cfg(test)]
mod tests {
    use crate::auxiliar;
    #[test]
    fn test_combination() {
        let combination1 = [
            [1, 2],
            [1, 3],
            [1, 4],
            [1, 5],
            [1, 6],
            [1, 7],
            [1, 8],
            [1, 9],
            [2, 3],
            [2, 4],
            [2, 5],
            [2, 6],
            [2, 7],
            [2, 8],
            [2, 9],
            [3, 4],
            [3, 5],
            [3, 6],
            [3, 7],
            [3, 8],
            [3, 9],
            [4, 5],
            [4, 6],
            [4, 7],
            [4, 8],
            [4, 9],
            [5, 6],
            [5, 7],
            [5, 8],
            [5, 9],
            [6, 7],
            [6, 8],
            [6, 9],
            [7, 8],
            [7, 9],
            [8, 9],
        ];
        let generated_combination = auxiliar::generate_combinations(2);
        assert_eq!(generated_combination, combination1);
    }
}
