use std::collections::{BTreeMap, HashSet};

/// [1, 2, 2, 3, 3, 3] -> 3 distinct numbers
pub fn unique_count(numbers: &[i32]) -> usize {
    // TODO: collect the numbers into a HashSet and return its len()
    todo!()
}

/// Numbers in both slices, sorted, no repeats. ([1, 2, 3, 3], [3, 2, 9]) -> [2, 3]
pub fn common(a: &[i32], b: &[i32]) -> Vec<i32> {
    // TODO: make a HashSet of each, take a.intersection(&b), collect to Vec, sort it
    todo!()
}

/// "Hello, World!" -> {'d': 1, 'e': 1, 'h': 1, 'l': 3, 'o': 2, 'r': 1, 'w': 1}
/// Only letters count, case is ignored. A BTreeMap keeps the keys sorted.
pub fn letter_counts(text: &str) -> BTreeMap<char, u32> {
    // TODO: BTreeMap::new(); for each alphabetic char: *counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_count_uses_a_set() {
        assert_eq!(unique_count(&[1, 2, 2, 3, 3, 3]), 3);
        assert_eq!(unique_count(&[]), 0);
    }

    #[test]
    fn common_uses_intersection() {
        assert_eq!(common(&[1, 2, 3, 3], &[3, 2, 9]), vec![2, 3]);
        assert_eq!(common(&[1], &[2]), Vec::<i32>::new());
    }

    #[test]
    fn letter_counts_are_sorted() {
        let counts = letter_counts("Hello, World!");
        assert_eq!(counts.get(&'l'), Some(&3));
        assert_eq!(counts.get(&'o'), Some(&2));
        assert_eq!(counts.get(&'!'), None, "only letters count");
        let keys: Vec<char> = counts.keys().copied().collect();
        assert_eq!(keys, vec!['d', 'e', 'h', 'l', 'o', 'r', 'w']);
    }
}
