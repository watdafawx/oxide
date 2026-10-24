use std::collections::HashSet;

/// How many different enchantments? [sharpness, unbreaking, sharpness] -> 2
pub fn distinct_enchants(list: &[&str]) -> usize {
    list.iter().collect::<HashSet<_>>().len()
}

/// Enchantments on both items, a to z, each once.
/// ([sharpness, unbreaking, mending], [unbreaking, mending, fortune]) -> [mending, unbreaking]
pub fn shared_enchants(sword: &[&str], axe: &[&str]) -> Vec<String> {
    let a: HashSet<&str> = sword.iter().copied().collect();
    let b: HashSet<&str> = axe.iter().copied().collect();
    let mut both: Vec<String> = a.intersection(&b).map(|e| e.to_string()).collect();
    both.sort();
    both
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
