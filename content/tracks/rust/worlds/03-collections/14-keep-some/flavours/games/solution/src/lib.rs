/// One line of a loot table: what drops, how rare it is (0 common to 3 legendary) and its weight.
#[derive(Debug, Clone, PartialEq)]
pub struct DropEntry {
    pub item: String,
    pub rarity: u8,
    pub weight: u32,
}

/// The weights of at least `min`: ([50, 2, 30], 10) -> [50, 30]
pub fn heavy_weights(weights: &[u32], min: u32) -> Vec<u32> {
    weights.iter().copied().filter(|w| *w >= min).collect()
}

/// The entries with `rarity >= min_rarity`, as references into the table.
pub fn rare_drops(table: &[DropEntry], min_rarity: u8) -> Vec<&DropEntry> {
    table.iter().filter(|d| d.rarity >= min_rarity).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item: &str, rarity: u8, weight: u32) -> DropEntry {
        DropEntry { item: item.to_string(), rarity, weight }
    }

    #[test]
    fn heavy_weights_keeps_weights_from_min() {
        assert_eq!(heavy_weights(&[50, 2, 30], 10), vec![50, 30]);
        assert_eq!(heavy_weights(&[2], 10), Vec::<u32>::new());
    }

    #[test]
    fn rare_drops_keeps_rare_entries() {
        let table = vec![entry("bone", 0, 30), entry("iron_ingot", 1, 10), entry("diamond", 3, 2)];
        assert_eq!(rare_drops(&table, 1), vec![&table[1], &table[2]], "iron and diamond, in table order");
        assert!(rare_drops(&table, 4).is_empty());
    }
}
