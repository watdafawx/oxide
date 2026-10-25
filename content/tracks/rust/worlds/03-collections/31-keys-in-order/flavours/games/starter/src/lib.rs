use std::collections::BTreeMap;

/// [torch, arrow, torch] -> {arrow: 1, torch: 2}, with the names sorted.
pub fn sorted_inventory(items: &[&str]) -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    for item in items {
        // TODO: add 1 to this item's count, starting at 0 the first time
    }
    counts
}

/// {arrow: 1, torch: 2} -> ["arrow x1", "torch x2"], a to z.
pub fn inventory_lines(counts: &BTreeMap<String, u32>) -> Vec<String> {
    // TODO: walk the (item, count) pairs in order and format each as "item xcount"
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_inventory_counts_in_a_sorted_map() {
        let inv = sorted_inventory(&["torch", "bread", "torch", "apple"]);
        assert_eq!(inv.get("torch"), Some(&2));
        let names: Vec<&String> = inv.keys().collect();
        assert_eq!(names, vec!["apple", "bread", "torch"], "a BTreeMap keeps the names sorted");
    }

    #[test]
    fn inventory_lines_walk_the_map_in_order() {
        let mut counts = BTreeMap::new();
        counts.insert("torch".to_string(), 2);
        counts.insert("arrow".to_string(), 1);
        assert_eq!(inventory_lines(&counts), vec!["arrow x1", "torch x2"]);
    }
}
