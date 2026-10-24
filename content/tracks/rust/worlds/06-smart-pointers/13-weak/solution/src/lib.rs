use std::rc::{Rc, Weak};

/// A link to the note that does not keep it alive.
pub fn watch(note: &Rc<String>) -> Weak<String> {
    Rc::downgrade(note)
}

/// The note's text while it still lives, or None once it is gone.
pub fn still_there(link: &Weak<String>) -> Option<String> {
    match link.upgrade() {
        Some(note) => Some(note.to_string()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watching_does_not_add_an_owner() {
        let note = Rc::new(String::from("Groceries"));
        let link = watch(&note);
        assert_eq!(Rc::strong_count(&note), 1, "a Weak is not an owner");
        assert_eq!(Rc::weak_count(&note), 1);
        assert!(link.upgrade().is_some(), "the link must point at the note");
    }

    #[test]
    fn still_there_is_none_once_the_note_is_gone() {
        let note = Rc::new(String::from("Groceries"));
        let link = Rc::downgrade(&note);
        assert_eq!(still_there(&link), Some(String::from("Groceries")));
        drop(note);
        assert_eq!(still_there(&link), None, "the note was freed, upgrade() gives None");
    }
}
