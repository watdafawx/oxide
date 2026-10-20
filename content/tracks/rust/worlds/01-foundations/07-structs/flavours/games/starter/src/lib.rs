pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    /// A new stack starts full: 64 items.
    pub fn new(name: &str) -> ItemStack {
        // TODO: build an ItemStack with `name.to_string()` and a count of 64
        todo!()
    }

    /// Remove `amount` items, but never go below 0.
    pub fn take(&mut self, amount: u32) {
        // TODO: lower self.count by `amount` without underflowing (saturating_sub, as in the lesson)
        todo!()
    }

    /// True while the stack still holds at least one item.
    pub fn has_items(&self) -> bool {
        // TODO: true when count is above 0
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cobblestone_stack_starts_full() {
        let stack = ItemStack::new("cobblestone");
        assert_eq!(stack.count, 64);
        assert_eq!(stack.name, "cobblestone");
    }

    #[test]
    fn taking_items_shrinks_the_stack() {
        let mut stack = ItemStack::new("cobblestone");
        stack.take(30);
        assert_eq!(stack.count, 34);
    }

    #[test]
    fn stack_count_stops_at_zero() {
        let mut stack = ItemStack::new("cobblestone");
        stack.take(250);
        assert_eq!(stack.count, 0, "count is a u32, so it must stop at 0");
    }

    #[test]
    fn has_items_only_while_count_positive() {
        let mut stack = ItemStack::new("cobblestone");
        assert!(stack.has_items(), "a fresh stack has items");
        stack.take(64);
        assert!(!stack.has_items(), "0 items means empty");
    }
}
