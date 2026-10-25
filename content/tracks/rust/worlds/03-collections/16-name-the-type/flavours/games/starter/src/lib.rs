/// One line of a loot table: what drops, how rare it is (0 common to 3 legendary) and its weight.
#[derive(Debug, Clone, PartialEq)]
pub struct DropEntry {
    pub item: String,
    pub rarity: u8,
    pub weight: u32,
}

/// The average weight, rounded down: weights 50, 30, 10 -> 30. An empty table gives 0.
pub fn average_weight(table: &[DropEntry]) -> u32 {
    if table.len() == 0 {
        return 0;
    }
    // TODO: sum the weights as a u32 (say the type on the sum) and divide by the number of entries
    0
}

/// How many full stacks of 64 the items make: [64, 32] -> 1.5
pub fn full_stacks_worth(items: &[u32]) -> f64 {
    // TODO: sum the items as a u32, turn the sum into an f64 and divide by 64.0
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item: &str, weight: u32) -> DropEntry {
        DropEntry { item: item.to_string(), rarity: 0, weight }
    }

    #[test]
    fn average_weight_divides_the_sum() {
        assert_eq!(average_weight(&[entry("flesh", 50), entry("bone", 30), entry("iron", 10)]), 30);
        assert_eq!(average_weight(&[entry("flesh", 50), entry("diamond", 2)]), 26);
        assert_eq!(average_weight(&[]), 0, "an empty table has no average");
    }

    #[test]
    fn full_stacks_worth_turns_items_into_stacks() {
        assert_eq!(full_stacks_worth(&[64, 32]), 1.5);
        assert_eq!(full_stacks_worth(&[16, 16, 32, 64]), 2.0);
    }
}
