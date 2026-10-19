/// Apply each percent-off in turn: 200.0 with [10.0, 50.0] -> 200 * 0.9 * 0.5 = 90.0
pub fn after_discounts(price: f64, percents: &[f64]) -> f64 {
    // TODO: percents.iter().fold(price, |current, off| current * (1.0 - off / 100.0))
    todo!()
}

/// The longest word. None for an empty list.
pub fn longest_word(words: &[String]) -> Option<&String> {
    // TODO: words.iter().max_by_key(|w| ...length...)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
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
