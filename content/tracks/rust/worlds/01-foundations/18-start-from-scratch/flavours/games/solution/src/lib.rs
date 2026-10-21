pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    /// A new stack starts full: 64 items.
    pub fn new(name: &str) -> ItemStack {
        ItemStack { name: name.to_string(), count: 64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_stacks_start_full() {
        let stack = ItemStack::new("cobblestone");
        assert_eq!(stack.count, 64);
    }

    #[test]
    fn new_keeps_the_name() {
        assert_eq!(ItemStack::new("cobblestone").name, "cobblestone");
        assert_eq!(ItemStack::new("torch").name, "torch");
    }
}
