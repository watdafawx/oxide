/// One line of a loot table: what drops, how rare it is (0 = common, 3 = legendary)
/// and how heavily it is weighted when the game rolls.
#[derive(Debug, Clone, PartialEq)]
pub struct Drop {
    pub item: String,
    pub rarity: u8,
    pub weight: u32,
}

/// The item names a table can drop, in order: [bone, arrow] and so on.
pub fn drop_names(table: &[Drop]) -> Vec<String> {
    table.iter().map(|d| d.item.clone()).collect()
}

/// Only the drops with `rarity >= min_rarity`.
pub fn rare_drops(table: &[Drop], min_rarity: u8) -> Vec<&Drop> {
    table.iter().filter(|d| d.rarity >= min_rarity).collect()
}

/// The sum of every weight: the number the game rolls against.
pub fn total_weight(table: &[Drop]) -> u32 {
    table.iter().map(|d| d.weight).sum()
}

/// How many drops have a weight of at least `min_weight`?
pub fn count_common(table: &[Drop], min_weight: u32) -> usize {
    table.iter().filter(|d| d.weight >= min_weight).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(item: &str, rarity: u8, weight: u32) -> Drop {
        Drop {
            item: item.to_string(),
            rarity,
            weight,
        }
    }

    fn zombie_table() -> Vec<Drop> {
        vec![
            entry("rotten_flesh", 0, 50),
            entry("bone", 0, 30),
            entry("iron_ingot", 1, 10),
            entry("diamond", 3, 2),
        ]
    }

    #[test]
    fn drop_names_maps_to_item_names() {
        assert_eq!(
            drop_names(&zombie_table()),
            vec!["rotten_flesh", "bone", "iron_ingot", "diamond"]
        );
        assert_eq!(drop_names(&[]), Vec::<String>::new());
    }

    #[test]
    fn rare_drops_keeps_rare_ones() {
        let table = zombie_table();
        let rare: Vec<&str> = rare_drops(&table, 1).iter().map(|d| d.item.as_str()).collect();
        assert_eq!(rare, vec!["iron_ingot", "diamond"]);
        assert!(rare_drops(&table, 4).is_empty());
        assert_eq!(rare_drops(&table, 0).len(), 4);
    }

    #[test]
    fn total_weight_sums_drops() {
        assert_eq!(total_weight(&zombie_table()), 92);
        assert_eq!(total_weight(&[]), 0);
    }

    #[test]
    fn count_common_filters_by_weight() {
        let table = zombie_table();
        assert_eq!(count_common(&table, 10), 3);
        assert_eq!(count_common(&table, 1), 4);
        assert_eq!(count_common(&table, 99), 0);
    }
}
