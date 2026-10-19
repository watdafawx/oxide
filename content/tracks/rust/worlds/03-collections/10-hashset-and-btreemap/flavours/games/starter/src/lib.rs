use std::collections::{BTreeMap, HashSet};

/// ["sharpness", "sharpness", "unbreaking"] -> 2 distinct enchantments
pub fn distinct_enchants(list: &[&str]) -> usize {
    // TODO: collect the list into a HashSet and return its len()
    todo!()
}

/// Enchantments on both items, sorted, no repeats.
/// (["sharpness", "unbreaking"], ["unbreaking", "mending"]) -> ["unbreaking"]
pub fn shared_enchants<'a>(a: &[&'a str], b: &[&'a str]) -> Vec<&'a str> {
    // TODO: make a HashSet of each, take set_a.intersection(&set_b), collect to Vec, sort it
    todo!()
}

/// ["torch", "arrow", "torch"] -> {"arrow": 1, "torch": 2}. A BTreeMap keeps the names sorted.
pub fn sorted_inventory(items: &[&str]) -> BTreeMap<String, u32> {
    // TODO: BTreeMap::new(); for each item: *counts.entry(item.to_string()).or_insert(0) += 1
    todo!()
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
