/// A pile of identical items: `ItemStack { name: "arrow", count: 24 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

/// The first stack size bigger than `limit`.
pub fn first_stack_over(stacks: &[u32], limit: u32) -> Option<u32> {
    stacks.iter().copied().find(|n| *n > limit)
}

/// Which slot holds `name`? None if it is not in the inventory.
pub fn slot_of(items: &[String], name: &str) -> Option<usize> {
    items.iter().position(|n| n == name)
}

/// Is any tool one hit from breaking (durability 1 or less)?
pub fn has_fragile_tool(durability: &[u32]) -> bool {
    durability.iter().any(|d| *d <= 1)
}

/// Is every stack at least `full` items? An empty list counts as yes.
pub fn all_full_stacks(counts: &[u32], full: u32) -> bool {
    counts.iter().all(|c| *c >= full)
}

/// Each armor piece trims a percent off what is left: 200.0 with [10.0, 50.0] -> 200 * 0.9 * 0.5 = 90.0
pub fn damage_after_armor(damage: f64, reductions: &[f64]) -> f64 {
    reductions.iter().fold(damage, |left, off| left * (1.0 - off / 100.0))
}

/// The stack with the most items. None for an empty inventory.
pub fn biggest_stack(stacks: &[ItemStack]) -> Option<&ItemStack> {
    stacks.iter().max_by_key(|s| s.count)
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
    fn find_and_position_search() {
        assert_eq!(first_stack_over(&[4, 8, 15, 16], 10), Some(15));
        assert_eq!(first_stack_over(&[4, 8], 10), None);
        let items = strings(&["sword", "bow", "torch"]);
        assert_eq!(slot_of(&items, "bow"), Some(1));
        assert_eq!(slot_of(&items, "elytra"), None);
    }

    #[test]
    fn any_and_all_check() {
        assert!(has_fragile_tool(&[30, 1, 12]));
        assert!(!has_fragile_tool(&[30, 2, 12]));
        assert!(all_full_stacks(&[64, 64, 70], 64));
        assert!(!all_full_stacks(&[64, 63], 64));
        assert!(all_full_stacks(&[], 64), "no stacks, nothing missing");
    }

    #[test]
    fn fold_builds_a_total() {
        let got = damage_after_armor(200.0, &[10.0, 50.0]);
        assert!((got - 90.0).abs() < 1e-9, "expected 90.0, got {got}");
        assert!((damage_after_armor(8.0, &[]) - 8.0).abs() < 1e-9, "no armor, full damage");
    }

    #[test]
    fn max_by_key_picks_longest() {
        let stacks = vec![stack("arrow", 24), stack("cobblestone", 64), stack("torch", 9)];
        assert_eq!(biggest_stack(&stacks), Some(&stacks[1]));
        assert_eq!(biggest_stack(&[]), None);
    }
}
