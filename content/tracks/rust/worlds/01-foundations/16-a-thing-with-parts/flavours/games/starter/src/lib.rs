pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

/// A full stack of cobblestone: 64 items.
pub fn cobblestone_stack() -> ItemStack {
    // TODO: ItemStack { name: ..., count: ... } (use `.to_string()` for the name)
    ItemStack { name: String::new(), count: 0 }
}

/// stack_label(stack) -> "cobblestone x64"
pub fn stack_label(stack: ItemStack) -> String {
    // TODO: format!("{} x{}", ...) with the name and the count
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_full_cobblestone_stack() {
        let stack = cobblestone_stack();
        assert_eq!(stack.name, "cobblestone");
        assert_eq!(stack.count, 64);
    }

    #[test]
    fn label_shows_name_and_count() {
        assert_eq!(stack_label(cobblestone_stack()), "cobblestone x64");
        let small = ItemStack { name: "torch".to_string(), count: 3 };
        assert_eq!(stack_label(small), "torch x3");
    }
}
