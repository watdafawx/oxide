use std::cell::RefCell;

/// True while some borrow of `note` is alive, so a borrow_mut() would panic.
pub fn is_busy(note: &RefCell<String>) -> bool {
    note.try_borrow_mut().is_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_busy_only_while_a_borrow_is_alive() {
        let note = RefCell::new(String::from("Groceries"));
        assert!(!is_busy(&note), "nobody holds the note yet");
        let reader = note.borrow();
        assert!(is_busy(&note), "a live borrow() blocks a writer");
        drop(reader);
        assert!(!is_busy(&note), "dropping the borrow frees the note");
        let editor = note.borrow_mut();
        assert!(is_busy(&note), "a live borrow_mut() blocks a writer too");
        drop(editor);
        assert!(!is_busy(&note), "and asking does not keep it busy");
    }
}
