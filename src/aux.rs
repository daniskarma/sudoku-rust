use std::collections::HashSet;

/// I use this function to check that all elements in a vector are unique.
pub fn all_unique<T: std::hash::Hash + std::cmp::Eq>(vec: &[T]) -> bool {
    let mut set = HashSet::new();
    vec.iter().all(move |x| set.insert(x))
}


