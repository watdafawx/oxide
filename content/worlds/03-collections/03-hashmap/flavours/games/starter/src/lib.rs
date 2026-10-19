use std::collections::HashMap;

/// Emeralds a villager wants for an item. Unknown items give None.
pub fn trade_price(prices: &HashMap<String, u32>, item: &str) -> Option<u32> {
    // TODO: prices.get(item) is Option<&u32>; .copied() makes it Option<u32>
    todo!()
}

/// Index a chest: item names to counts. Same position in both slices = one pair.
pub fn chest_index(items: &[&str], counts: &[u32]) -> HashMap<String, u32> {
    // TODO: make a HashMap, then insert (item.to_string(), *count) for each pair
    // TODO: `for (item, count) in items.iter().zip(counts)` walks both slices together
    todo!()
}

/// "zombie_flesh bone zombie_flesh" -> {"zombie_flesh": 2, "bone": 1}. Items are split on whitespace.
pub fn count_drops(log: &str) -> HashMap<String, u32> {
    // TODO: for each item in log.split_whitespace(): *counts.entry(item.to_string()).or_insert(0) += 1
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_of_uses_get() {
        let mut prices = HashMap::new();
        prices.insert("bread".to_string(), 1);
        assert_eq!(trade_price(&prices, "bread"), Some(1));
        assert_eq!(trade_price(&prices, "elytra"), None);
    }

    #[test]
    fn stock_map_inserts_pairs() {
        let index = chest_index(&["diamond", "coal"], &[3, 40]);
        assert_eq!(index.len(), 2);
        assert_eq!(index.get("diamond"), Some(&3));
        assert_eq!(index.get("coal"), Some(&40));
    }

    #[test]
    fn count_words_counts_each_word() {
        let counts = count_drops("bone flesh bone arrow bone");
        assert_eq!(counts.get("bone"), Some(&3), "bone dropped 3 times");
        assert_eq!(counts.get("flesh"), Some(&1));
        assert_eq!(counts.len(), 3);
        assert!(count_drops("").is_empty());
    }
}
