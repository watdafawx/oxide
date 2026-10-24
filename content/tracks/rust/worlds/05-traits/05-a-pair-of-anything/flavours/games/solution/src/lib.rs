pub struct Hands<T> {
    pub main_hand: T,
    pub off_hand: T,
}

impl<T> Hands<T> {
    pub fn new(main_hand: T, off_hand: T) -> Self {
        Hands { main_hand, off_hand }
    }

    /// Same items, hands swapped. Uses up the old value.
    pub fn swap_hands(self) -> Hands<T> {
        Hands {
            main_hand: self.off_hand,
            off_hand: self.main_hand,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hands_hold_any_item() {
        let stacks = Hands::new(64, 16);
        assert_eq!((stacks.main_hand, stacks.off_hand), (64, 16));
        let blocks = Hands::new("torch", "dirt");
        assert_eq!((blocks.main_hand, blocks.off_hand), ("torch", "dirt"));
    }

    #[test]
    fn swap_hands_crosses_them_over() {
        let h = Hands::new("sword", "shield").swap_hands();
        assert_eq!((h.main_hand, h.off_hand), ("shield", "sword"));
        let owned = Hands::new(String::from("pickaxe"), String::from("torch")).swap_hands();
        assert_eq!(owned.main_hand, "torch", "works for owned Strings too, no clone needed");
    }
}
