/// A pile of identical items: `ItemStack { name: "arrow", count: 24 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

/// Pull the item out of `slot` and put it first (hotbar slot 1). [a, b, c] with slot 2 -> [c, a, b].
pub fn move_to_hotbar(inventory: &mut Vec<String>, slot: usize) {
    let item = inventory.remove(slot);
    inventory.insert(0, item);
}

/// Sorted by name, with repeats removed. [bone, arrow, bone] -> [arrow, bone].
pub fn tidy_loot(mut drops: Vec<String>) -> Vec<String> {
    drops.sort();
    drops.dedup();
    drops
}

/// Is the exact item "elytra" in the inventory?
pub fn has_elytra(items: &[String]) -> bool {
    items.contains(&"elytra".to_string())
}

/// Trash every stack with fewer than `min_count` items.
pub fn drop_small_stacks(stacks: &mut Vec<ItemStack>, min_count: u32) {
    stacks.retain(|s| s.count >= min_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    fn stack(name: &str, count: u32) -> ItemStack {
        ItemStack {
            name: name.to_string(),
            count,
        }
    }

    #[test]
    fn move_to_hotbar_puts_item_first() {
        let mut inventory = strings(&["sword", "bow", "pickaxe"]);
        move_to_hotbar(&mut inventory, 2);
        assert_eq!(inventory, strings(&["pickaxe", "sword", "bow"]));
        move_to_hotbar(&mut inventory, 0);
        assert_eq!(inventory, strings(&["pickaxe", "sword", "bow"]), "slot 0 changes nothing");
    }

    #[test]
    fn tidy_loot_sorts_and_dedups() {
        assert_eq!(
            tidy_loot(strings(&["bone", "arrow", "bone", "string", "arrow"])),
            strings(&["arrow", "bone", "string"])
        );
        assert_eq!(tidy_loot(vec![]), Vec::<String>::new());
    }

    #[test]
    fn has_elytra_checks_inventory() {
        assert!(has_elytra(&strings(&["dirt", "elytra"])));
        assert!(!has_elytra(&strings(&["dirt", "elytra_wing"])));
    }

    #[test]
    fn drop_small_stacks_keeps_big_ones() {
        let mut stacks = vec![stack("dirt", 1), stack("iron_ingot", 12), stack("flint", 2), stack("bread", 5)];
        drop_small_stacks(&mut stacks, 3);
        assert_eq!(stacks, vec![stack("iron_ingot", 12), stack("bread", 5)]);
    }
}
