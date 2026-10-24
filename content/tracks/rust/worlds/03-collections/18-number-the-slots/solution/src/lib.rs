/// [pen, mug] -> ["1. pen", "2. mug"]: lines count from 1.
pub fn numbered(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .enumerate()
        .map(|(i, name)| format!("{}. {name}", i + 1))
        .collect()
}

/// The positions (from 0) of the counts under `limit`: ([12, 2, 6, 1], 5) -> [1, 3]
pub fn low_shelves(counts: &[u32], limit: u32) -> Vec<usize> {
    counts
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, n)| *n < limit)
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_counts_from_one() {
        assert_eq!(numbered(&["pen", "mug"]), vec!["1. pen", "2. mug"]);
        assert_eq!(numbered(&[]), Vec::<String>::new());
    }

    #[test]
    fn low_shelves_gives_positions() {
        assert_eq!(low_shelves(&[12, 2, 6, 1], 5), vec![1, 3]);
        assert_eq!(low_shelves(&[12, 6], 5), Vec::<usize>::new());
    }
}
