/// The first number bigger than `limit`.
pub fn first_over(numbers: &[i32], limit: i32) -> Option<i32> {
    numbers.iter().copied().find(|n| *n > limit)
}

/// Where in the list is `name`? None if it is not there.
pub fn index_of(names: &[String], name: &str) -> Option<usize> {
    names.iter().position(|n| n.as_str() == name)
}

/// Is any number below zero?
pub fn has_negative(numbers: &[i32]) -> bool {
    numbers.iter().any(|n| *n < 0)
}

/// Does every score reach `pass`? An empty list counts as yes.
pub fn all_passing(scores: &[u32], pass: u32) -> bool {
    scores.iter().all(|s| *s >= pass)
}

/// Apply each percent-off in turn: 200.0 with [10.0, 50.0] -> 200 * 0.9 * 0.5 = 90.0
pub fn after_discounts(price: f64, percents: &[f64]) -> f64 {
    percents.iter().fold(price, |current, off| current * (1.0 - off / 100.0))
}

/// The longest word. None for an empty list.
pub fn longest_word(words: &[String]) -> Option<&String> {
    words.iter().max_by_key(|w| w.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn find_and_position_search() {
        assert_eq!(first_over(&[4, 8, 15, 16], 10), Some(15));
        assert_eq!(first_over(&[4, 8], 10), None);
        let names = strings(&["ann", "bo", "cy"]);
        assert_eq!(index_of(&names, "bo"), Some(1));
        assert_eq!(index_of(&names, "zed"), None);
    }

    #[test]
    fn any_and_all_check() {
        assert!(has_negative(&[3, -1, 2]));
        assert!(!has_negative(&[3, 0, 2]));
        assert!(all_passing(&[60, 75, 90], 60));
        assert!(!all_passing(&[60, 59], 60));
        assert!(all_passing(&[], 60), "no scores, nothing failing");
    }

    #[test]
    fn fold_builds_a_total() {
        let got = after_discounts(200.0, &[10.0, 50.0]);
        assert!((got - 90.0).abs() < 1e-9, "expected 90.0, got {got}");
        assert!((after_discounts(80.0, &[]) - 80.0).abs() < 1e-9, "no discounts, same price");
    }

    #[test]
    fn max_by_key_picks_longest() {
        let words = strings(&["pear", "banana", "fig"]);
        assert_eq!(longest_word(&words), Some(&"banana".to_string()));
        assert_eq!(longest_word(&[]), None);
    }
}
