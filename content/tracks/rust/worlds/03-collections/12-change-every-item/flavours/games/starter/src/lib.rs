/// One line of a loot table: what drops, how rare it is (0 common to 3 legendary) and its weight.
#[derive(Debug, Clone, PartialEq)]
pub struct DropEntry {
    pub item: String,
    pub rarity: u8,
    pub weight: u32,
}

/// Double-drops event: every count times two. [1, 3] -> [2, 6]
pub fn doubled_drops(drops: &[u32]) -> Vec<u32> {
    // TODO: one chain: walk the counts, double each one, collect into a Vec
    Vec::new()
}

/// The item names in the table, in order: [rotten_flesh, bone, ...]
pub fn drop_names(table: &[DropEntry]) -> Vec<String> {
    // TODO: one chain: walk the entries, map each to a copy of its item name, collect
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item: &str, rarity: u8, weight: u32) -> DropEntry {
        DropEntry { item: item.to_string(), rarity, weight }
    }

    #[test]
    fn doubled_drops_double_every_count() {
        assert_eq!(doubled_drops(&[1, 3]), vec![2, 6]);
        assert_eq!(doubled_drops(&[]), Vec::<u32>::new());
    }

    #[test]
    fn drop_names_list_every_item() {
        let table = vec![entry("rotten_flesh", 0, 50), entry("diamond", 3, 2)];
        assert_eq!(drop_names(&table), vec!["rotten_flesh", "diamond"]);
        assert_eq!(table.len(), 2, "the table is untouched");
    }
}
