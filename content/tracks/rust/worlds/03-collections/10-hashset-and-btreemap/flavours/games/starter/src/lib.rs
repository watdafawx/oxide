use std::collections::{BTreeMap, HashSet};

/// ["sharpness", "sharpness", "unbreaking"] -> 2 distinct enchantments
pub fn distinct_enchants(list: &[&str]) -> usize {
    // TODO: put the items in a HashSet (it drops repeats) and return its size
    todo!()
}

/// Enchantments on both items, sorted, no repeats.
/// (["sharpness", "unbreaking"], ["unbreaking", "mending"]) -> ["unbreaking"]
pub fn shared_enchants<'a>(a: &[&'a str], b: &[&'a str]) -> Vec<&'a str> {
    // TODO: find what both lists share (sets have an `intersection`), collect into a Vec and sort it
    todo!()
}

/// ["torch", "arrow", "torch"] -> {"arrow": 1, "torch": 2}. A BTreeMap keeps the names sorted.
pub fn sorted_inventory(items: &[&str]) -> BTreeMap<String, u32> {
    // TODO: count each item in a BTreeMap so the names come out in order
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
