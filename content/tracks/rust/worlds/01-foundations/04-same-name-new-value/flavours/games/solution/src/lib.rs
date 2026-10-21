#![allow(clippy::let_and_return)]
/// How many letters an item name has once the spaces around it are gone.
/// item_name_len("  oak_log ") -> 7
pub fn item_name_len(name: &str) -> usize {
    let name = name.trim();
    name.len()
}

/// Two crafting tables are made, and each costs 4 planks.
pub fn planks_after_two_crafts(planks: u32) -> u32 {
    let planks = planks - 4;
    let planks = planks - 4;
    planks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_is_not_counted() {
        assert_eq!(item_name_len("  oak_log "), 7);
        assert_eq!(item_name_len("stone"), 5);
    }

    #[test]
    fn two_crafts_use_eight_planks() {
        assert_eq!(planks_after_two_crafts(48), 40);
        assert_eq!(planks_after_two_crafts(10), 2);
    }
}
