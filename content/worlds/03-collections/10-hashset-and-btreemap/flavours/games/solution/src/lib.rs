use std::collections::{BTreeMap, HashSet};

/// ["sharpness", "sharpness", "unbreaking"] -> 2 distinct enchantments
pub fn distinct_enchants(list: &[&str]) -> usize {
    list.iter().collect::<HashSet<_>>().len()
}

/// Enchantments on both items, sorted, no repeats.
/// (["sharpness", "unbreaking"], ["unbreaking", "mending"]) -> ["unbreaking"]
pub fn shared_enchants<'a>(a: &[&'a str], b: &[&'a str]) -> Vec<&'a str> {
    let set_a: HashSet<&str> = a.iter().copied().collect();
    let set_b: HashSet<&str> = b.iter().copied().collect();
    let mut shared: Vec<&str> = set_a.intersection(&set_b).copied().collect();
    shared.sort();
    shared
}

/// ["torch", "arrow", "torch"] -> {"arrow": 1, "torch": 2}. A BTreeMap keeps the names sorted.
pub fn sorted_inventory(items: &[&str]) -> BTreeMap<String, u32> {
    let mut counts = BTreeMap::new();
    for item in items {
        *counts.entry(item.to_string()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_enchants_ignores_repeats() {
        assert_eq!(distinct_enchants(&["sharpness", "sharpness", "unbreaking"]), 2);
        assert_eq!(distinct_enchants(&[]), 0);
    }

    #[test]
    fn shared_enchants_are_on_both_items() {
        assert_eq!(
            shared_enchants(&["sharpness", "unbreaking", "mending"], &["unbreaking", "mending", "fortune"]),
            vec!["mending", "unbreaking"]
        );
        assert_eq!(shared_enchants(&["sharpness"], &["fortune"]), Vec::<&str>::new());
    }

    #[test]
    fn sorted_inventory_lists_a_to_z() {
        let inventory = sorted_inventory(&["torch", "bread", "torch", "apple", "torch"]);
        assert_eq!(inventory.get("torch"), Some(&3));
        assert_eq!(inventory.get("bread"), Some(&1));
        assert_eq!(inventory.get("elytra"), None);
        let names: Vec<&str> = inventory.keys().map(|k| k.as_str()).collect();
        assert_eq!(names, vec!["apple", "bread", "torch"]);
    }
}
