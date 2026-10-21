pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    pub fn new(name: &str) -> ItemStack {
        ItemStack { name: name.to_string(), count: 64 }
    }

    /// Remove `amount` items. The count never goes below zero.
    // TODO: set self.count to self.count.saturating_sub(amount). When Rust complains, change `&self`.
    pub fn take(&self, amount: u32) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taking_removes_items() {
        let mut stack = ItemStack::new("cobblestone");
        stack.take(30);
        assert_eq!(stack.count, 34);
    }

    #[test]
    fn taking_too_many_stops_at_zero() {
        let mut stack = ItemStack::new("cobblestone");
        stack.take(100);
        assert_eq!(stack.count, 0, "count is a u32, so it must stop at 0");
    }
}
