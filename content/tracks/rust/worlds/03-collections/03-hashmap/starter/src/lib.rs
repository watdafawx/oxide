use std::collections::HashMap;

/// Look up an item's price. Unknown items give None.
pub fn price_of(prices: &HashMap<String, f64>, item: &str) -> Option<f64> {
    // TODO: prices.get(item) is Option<&f64>; .copied() makes it Option<f64>
    todo!()
}

/// Build a map from names to counts. Same position in both slices = one pair.
pub fn stock_map(names: &[&str], counts: &[u32]) -> HashMap<String, u32> {
    // TODO: make a HashMap, then insert (name.to_string(), *count) for each pair
    // `for (name, count) in names.iter().zip(counts)` walks both slices together
    todo!()
}

/// "the cat the hat" -> {"the": 2, "cat": 1, "hat": 1}. Words are split on whitespace.
pub fn count_words(text: &str) -> HashMap<String, u32> {
    // TODO: for each word in text.split_whitespace(): *counts.entry(word.to_string()).or_insert(0) += 1
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_of_uses_get() {
        let mut prices = HashMap::new();
        prices.insert("apple".to_string(), 0.5);
        assert_eq!(price_of(&prices, "apple"), Some(0.5));
        assert_eq!(price_of(&prices, "pear"), None);
    }

    #[test]
    fn stock_map_inserts_pairs() {
        let map = stock_map(&["nails", "screws"], &[100, 40]);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("nails"), Some(&100));
        assert_eq!(map.get("screws"), Some(&40));
    }

    #[test]
    fn count_words_counts_each_word() {
        let counts = count_words("the cat the hat the end");
        assert_eq!(counts.get("the"), Some(&3), "'the' appears 3 times");
        assert_eq!(counts.get("cat"), Some(&1));
        assert_eq!(counts.len(), 4);
        assert!(count_words("").is_empty());
    }
}
