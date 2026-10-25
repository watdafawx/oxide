/// [sword, bow] -> ["[1] sword", "[2] bow"]: slots count from 1, like the number keys.
pub fn hotbar_labels(items: &[&str]) -> Vec<String> {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| format!("[{}] {item}", i + 1))
        .collect()
}

/// The positions (from 0) of the empty slots: [64, 0, 16, 0] -> [1, 3]
pub fn empty_slots(stacks: &[u32]) -> Vec<usize> {
    stacks
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, n)| *n == 0)
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotbar_labels_count_from_one() {
        assert_eq!(hotbar_labels(&["sword", "bow"]), vec!["[1] sword", "[2] bow"]);
        assert_eq!(hotbar_labels(&[]), Vec::<String>::new());
    }

    #[test]
    fn empty_slots_gives_positions() {
        assert_eq!(empty_slots(&[64, 0, 16, 0]), vec![1, 3]);
        assert_eq!(empty_slots(&[64, 16]), Vec::<usize>::new());
    }
}
