use std::cell::RefCell;

/// True while some borrow of `chest` is alive, so a borrow_mut() would panic.
pub fn chest_busy(chest: &RefCell<Vec<String>>) -> bool {
    // TODO: would try_borrow_mut() on the chest give an Err?
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chest_busy_only_while_a_borrow_is_alive() {
        let chest = RefCell::new(vec![String::from("iron-plate")]);
        assert!(!chest_busy(&chest), "nobody holds the chest yet");
        let reader = chest.borrow();
        assert!(chest_busy(&chest), "a live borrow() blocks a writer");
        drop(reader);
        assert!(!chest_busy(&chest), "dropping the borrow frees the chest");
        let inserter = chest.borrow_mut();
        assert!(chest_busy(&chest), "a live borrow_mut() blocks a writer too");
        drop(inserter);
        assert!(!chest_busy(&chest), "and asking does not keep it busy");
    }
}
