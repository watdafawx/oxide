pub struct ItemStack {
    pub name: String,
    pub count: u32,
}

impl ItemStack {
    /// True when there are no items at all.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// True when the stack holds exactly 64 items.
    pub fn is_full(&self) -> bool {
        self.count == 64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack(n: u32) -> ItemStack {
        ItemStack { name: "stone".to_string(), count: n }
    }

    #[test]
    fn an_empty_stack_is_empty() {
        assert!(stack(0).is_empty());
        assert!(!stack(1).is_empty());
    }

    #[test]
    fn only_64_items_is_full() {
        assert!(stack(64).is_full());
        assert!(!stack(63).is_full());
        assert!(!stack(0).is_full());
    }
}
