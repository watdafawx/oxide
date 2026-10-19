pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    /// A new stack starts full: 64 items.
    pub fn new(name: &str) -> ItemStack {
        ItemStack {
            name: name.to_string(),
            count: 64,
        }
    }

    /// Remove `amount` items, but never go below 0.
    pub fn take(&mut self, amount: u32) {
        self.count = self.count.saturating_sub(amount);
    }

    /// True while the stack still holds at least one item.
    pub fn has_items(&self) -> bool {
        self.count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_full_health() {
        let stack = ItemStack::new("cobblestone");
        assert_eq!(stack.count, 64);
        assert_eq!(stack.name, "cobblestone");
    }

    #[test]
    fn take_hit_lowers_hp() {
        let mut stack = ItemStack::new("cobblestone");
        stack.take(30);
        assert_eq!(stack.count, 34);
    }

    #[test]
    fn hp_never_goes_below_zero() {
        let mut stack = ItemStack::new("cobblestone");
        stack.take(250);
        assert_eq!(stack.count, 0, "count is a u32, so it must stop at 0");
    }

    #[test]
    fn is_alive_checks_hp() {
        let mut stack = ItemStack::new("cobblestone");
        assert!(stack.has_items(), "a fresh stack has items");
        stack.take(64);
        assert!(!stack.has_items(), "0 items means empty");
    }
}
