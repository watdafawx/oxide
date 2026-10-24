use std::cell::RefCell;

/// Counts swings. `swing` takes &self, so the count lives in a RefCell.
pub struct Inserter {
    swings: RefCell<u32>,
}

impl Inserter {
    pub fn new() -> Inserter {
        Inserter {
            swings: RefCell::new(0),
        }
    }

    pub fn swing(&self) {
        *self.swings.borrow_mut() += 1;
    }

    pub fn swings(&self) -> u32 {
        *self.swings.borrow()
    }
}

impl Default for Inserter {
    fn default() -> Self {
        Self::new()
    }
}

/// Appends `item` to the chest's inventory behind the RefCell.
pub fn put_in_chest(chest: &RefCell<Vec<String>>, item: &str) {
    chest.borrow_mut().push(item.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swing_counts_through_shared_ref() {
        let inserter = Inserter::new();
        let shared = &inserter;
        shared.swing();
        shared.swing();
        assert_eq!(inserter.swings(), 2);
    }

    #[test]
    fn put_in_chest_pushes_item() {
        let chest = RefCell::new(vec![String::from("iron-plate")]);
        put_in_chest(&chest, "copper-plate");
        assert_eq!(*chest.borrow(), vec!["iron-plate", "copper-plate"]);
    }
}
