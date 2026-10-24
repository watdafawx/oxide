use std::collections::HashSet;

/// How many different enchantments? [sharpness, unbreaking, sharpness] -> 2
pub fn distinct_enchants(list: &[&str]) -> usize {
    // TODO: collect the list into a HashSet (repeats vanish) and give its length
    list.len()
}

/// Enchantments on both items, a to z, each once.
/// ([sharpness, unbreaking, mending], [unbreaking, mending, fortune]) -> [mending, unbreaking]
pub fn shared_enchants(sword: &[&str], axe: &[&str]) -> Vec<String> {
    // TODO: make a HashSet of each list, intersect them, map each to a String, collect, then sort
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_enchants_ignores_repeats() {
        assert_eq!(distinct_enchants(&["sharpness", "unbreaking", "sharpness"]), 2);
        assert_eq!(distinct_enchants(&[]), 0);
    }

    #[test]
    fn shared_enchants_lists_common_ones() {
        assert_eq!(
            shared_enchants(&["sharpness", "unbreaking", "mending"], &["unbreaking", "mending", "fortune"]),
            vec!["mending", "unbreaking"]
        );
        assert_eq!(shared_enchants(&["sharpness"], &["fortune"]), Vec::<String>::new());
    }
}
