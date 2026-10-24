/// A pile of identical items: `ItemStack { name: "arrow", count: 24 }`.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

/// The most items first: cobblestone 64, arrow 24, torch 9.
pub fn biggest_first(stacks: &mut Vec<ItemStack>) {
    stacks.sort_by(|a, b| b.count.cmp(&a.count));
}

/// By name, a to z: arrow, cobblestone, torch.
pub fn by_name(stacks: &mut Vec<ItemStack>) {
    stacks.sort_by(|a, b| a.name.cmp(&b.name));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack(name: &str, count: u32) -> ItemStack {
        ItemStack { name: name.to_string(), count }
    }

    fn chest() -> Vec<ItemStack> {
        vec![stack("torch", 9), stack("cobblestone", 64), stack("arrow", 24)]
    }

    #[test]
    fn biggest_first_sorts_by_count_descending() {
        let mut stacks = chest();
        biggest_first(&mut stacks);
        assert_eq!(stacks, vec![stack("cobblestone", 64), stack("arrow", 24), stack("torch", 9)]);
    }

    #[test]
    fn by_name_sorts_a_to_z() {
        let mut stacks = chest();
        by_name(&mut stacks);
        assert_eq!(stacks, vec![stack("arrow", 24), stack("cobblestone", 64), stack("torch", 9)]);
    }
}
