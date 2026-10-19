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

/// True if `chest` is currently borrowed so that a borrow_mut() would panic.
pub fn chest_busy(chest: &RefCell<Vec<String>>) -> bool {
    chest.try_borrow_mut().is_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_counts_through_shared_ref() {
        let i = Inserter::new();
        let r = &i;
        r.swing();
        r.swing();
        assert_eq!(i.swings(), 2);
    }

    #[test]
    fn add_item_mutates_in_place() {
        let chest = RefCell::new(vec![String::from("iron-plate")]);
        put_in_chest(&chest, "copper-plate");
        assert_eq!(*chest.borrow(), vec!["iron-plate", "copper-plate"]);
    }

    #[test]
    fn is_locked_detects_active_borrow() {
        let chest = RefCell::new(vec![String::from("iron-plate")]);
        assert!(!chest_busy(&chest), "a free chest is not busy");
        let reader = chest.borrow();
        assert!(chest_busy(&chest), "a live borrow() locks out borrow_mut()");
        drop(reader);
        assert!(!chest_busy(&chest), "dropping the borrow frees the chest");
    }
}
