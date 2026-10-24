/// One line of a loot table: what drops, how rare it is (0 common to 3 legendary) and its weight.
#[derive(Debug, Clone, PartialEq)]
pub struct DropEntry {
    pub item: String,
    pub rarity: u8,
    pub weight: u32,
}

/// Looting: every weight doubles, in the caller's table.
pub fn apply_looting(table: &mut Vec<DropEntry>) {
    for d in table.iter_mut() {
        d.weight *= 2;
    }
}

/// The item names, in order. The table is not needed afterwards, so take it over.
pub fn into_item_names(table: Vec<DropEntry>) -> Vec<String> {
    table.into_iter().map(|d| d.item).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item: &str, rarity: u8, weight: u32) -> DropEntry {
        DropEntry { item: item.to_string(), rarity, weight }
    }

    #[test]
    fn apply_looting_changes_the_table_in_place() {
        let mut table = vec![entry("bone", 0, 30), entry("diamond", 3, 2)];
        apply_looting(&mut table);
        assert_eq!(table, vec![entry("bone", 0, 60), entry("diamond", 3, 4)]);
    }

    #[test]
    fn into_item_names_takes_the_names_without_copying() {
        let table = vec![entry("bone", 0, 30), entry("diamond", 3, 2)];
        assert_eq!(into_item_names(table), vec!["bone", "diamond"]);
        assert_eq!(into_item_names(Vec::new()), Vec::<String>::new());
    }
}
