/// One line of a loot table: what drops, how rare it is (0 common to 3 legendary) and its weight.
#[derive(Debug, Clone, PartialEq)]
pub struct DropEntry {
    pub item: String,
    pub rarity: u8,
    pub weight: u32,
}

/// The number the game rolls against: every weight added up.
pub fn total_weight(table: &[DropEntry]) -> u32 {
    // TODO: one chain: pick each entry's weight, add them up
    0
}

/// How many entries weigh at least `min_weight`?
pub fn count_common(table: &[DropEntry], min_weight: u32) -> usize {
    // TODO: one chain: keep the entries that weigh at least `min_weight`, then count them
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item: &str, rarity: u8, weight: u32) -> DropEntry {
        DropEntry { item: item.to_string(), rarity, weight }
    }

    fn zombie() -> Vec<DropEntry> {
        vec![entry("rotten_flesh", 0, 50), entry("bone", 0, 30), entry("iron_ingot", 1, 10), entry("diamond", 3, 2)]
    }

    #[test]
    fn total_weight_adds_every_entry() {
        assert_eq!(total_weight(&zombie()), 92);
        assert_eq!(total_weight(&[]), 0, "an empty table weighs 0");
    }

    #[test]
    fn count_common_counts_with_a_chain() {
        assert_eq!(count_common(&zombie(), 10), 3);
        assert_eq!(count_common(&zombie(), 99), 0);
    }
}
