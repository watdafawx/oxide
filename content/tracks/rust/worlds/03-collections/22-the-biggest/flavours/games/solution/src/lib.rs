/// A pile of identical items: `ItemStack { name: "arrow", count: 24 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

/// The stack with the most items. None for an empty inventory.
pub fn biggest_stack(stacks: &[ItemStack]) -> Option<&ItemStack> {
    stacks.iter().max_by_key(|s| s.count)
}

/// The shortest item name, for the compact tooltip. None for an empty list.
pub fn shortest_name(names: &[String]) -> Option<&String> {
    names.iter().min_by_key(|n| n.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack(name: &str, count: u32) -> ItemStack {
        ItemStack { name: name.to_string(), count }
    }

    #[test]
    fn biggest_stack_picks_by_count() {
        let stacks = vec![stack("arrow", 24), stack("cobblestone", 64), stack("torch", 9)];
        assert_eq!(biggest_stack(&stacks), Some(&stacks[1]));
        assert_eq!(biggest_stack(&[]), None);
    }

    #[test]
    fn shortest_name_picks_by_length() {
        let names = vec!["diamond_sword".to_string(), "bow".to_string(), "torch".to_string()];
        assert_eq!(shortest_name(&names), Some(&names[1]));
        assert_eq!(shortest_name(&[]), None);
    }
}
