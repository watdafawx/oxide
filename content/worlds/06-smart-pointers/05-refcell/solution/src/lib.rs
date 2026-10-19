use std::cell::RefCell;

/// Counts hits. `hit` takes &self, so the count lives in a RefCell.
pub struct Counter {
    hits: RefCell<u32>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            hits: RefCell::new(0),
        }
    }

    pub fn hit(&self) {
        *self.hits.borrow_mut() += 1;
    }

    pub fn get(&self) -> u32 {
        *self.hits.borrow()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Appends `item` to the list behind the RefCell.
pub fn add_item(list: &RefCell<Vec<String>>, item: &str) {
    list.borrow_mut().push(item.to_string());
}

/// True if `cell` is currently borrowed so that a borrow_mut() would panic.
pub fn is_locked(cell: &RefCell<i32>) -> bool {
    cell.try_borrow_mut().is_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_counts_through_shared_ref() {
        let c = Counter::new();
        let r = &c;
        r.hit();
        r.hit();
        assert_eq!(c.get(), 2);
    }

    #[test]
    fn add_item_mutates_in_place() {
        let list = RefCell::new(vec![String::from("a")]);
        add_item(&list, "b");
        assert_eq!(*list.borrow(), vec!["a", "b"]);
    }

    #[test]
    fn is_locked_detects_active_borrow() {
        let cell = RefCell::new(1);
        assert!(!is_locked(&cell), "a free cell is not locked");
        let reader = cell.borrow();
        assert!(is_locked(&cell), "a live borrow() locks out borrow_mut()");
        drop(reader);
        assert!(!is_locked(&cell), "dropping the borrow unlocks it");
    }
}
