use std::fmt::Display;

/// Two held items: main hand and off hand. Works for any kind of item value.
pub struct Inventory<T> {
    pub main_hand: T,
    pub off_hand: T,
}

impl<T> Inventory<T> {
    pub fn new(main_hand: T, off_hand: T) -> Self {
        Inventory { main_hand, off_hand }
    }

    /// Same items, hands swapped.
    pub fn swap_hands(self) -> Inventory<T> {
        Inventory {
            main_hand: self.off_hand,
            off_hand: self.main_hand,
        }
    }
}

impl<T: PartialOrd> Inventory<T> {
    /// The better of the two. Ties return `main_hand`.
    pub fn best(&self) -> &T {
        if self.main_hand >= self.off_hand {
            &self.main_hand
        } else {
            &self.off_hand
        }
    }
}

impl<T: Display + PartialOrd> Inventory<T> {
    pub fn describe_best(&self) -> String {
        format!("Best item: {}", self.best())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_new_and_swap_hands() {
        let inv = Inventory::new("sword", "shield").swap_hands();
        assert_eq!((inv.main_hand, inv.off_hand), ("shield", "sword"));
        let nums = Inventory::new(1, 2);
        assert_eq!((nums.main_hand, nums.off_hand), (1, 2));
    }

    #[test]
    fn best_picks_the_better_item() {
        assert_eq!(*Inventory::new(3, 8).best(), 8);
        assert_eq!(*Inventory::new(2.5, 1.0).best(), 2.5);
    }

    #[test]
    fn describe_best_formats_text() {
        assert_eq!(Inventory::new(3, 8).describe_best(), "Best item: 8");
        assert_eq!(Inventory::new("pickaxe", "apple").describe_best(), "Best item: pickaxe");
    }
}
