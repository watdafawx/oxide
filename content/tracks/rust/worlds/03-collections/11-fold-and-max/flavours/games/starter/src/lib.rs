/// A pile of identical items: `ItemStack { name: "arrow", count: 24 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

/// Each armor piece trims a percent off what is left: 200.0 with [10.0, 50.0] -> 200 * 0.9 * 0.5 = 90.0
pub fn damage_after_armor(damage: f64, reductions: &[f64]) -> f64 {
    // TODO: reductions.iter().fold(damage, |left, off| left * (1.0 - off / 100.0))
    todo!()
}

/// The stack with the most items. None for an empty inventory.
pub fn biggest_stack(stacks: &[ItemStack]) -> Option<&ItemStack> {
    // TODO: stacks.iter().max_by_key(|s| ...the count...)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack(name: &str, count: u32) -> ItemStack {
        ItemStack {
            name: name.to_string(),
            count,
        }
    }

    #[test]
    fn armor_reduces_damage_step_by_step() {
        let got = damage_after_armor(200.0, &[10.0, 50.0]);
        assert!((got - 90.0).abs() < 1e-9, "expected 90.0, got {got}");
        assert!((damage_after_armor(8.0, &[]) - 8.0).abs() < 1e-9, "no armor, full damage");
    }

    #[test]
    fn biggest_stack_has_most_items() {
        let stacks = vec![stack("arrow", 24), stack("cobblestone", 64), stack("torch", 9)];
        assert_eq!(biggest_stack(&stacks), Some(&stacks[1]));
        assert_eq!(biggest_stack(&[]), None);
    }
}
