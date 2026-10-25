use std::cell::RefCell;
use std::rc::Rc;

/// A shared, changeable note: several owners, edited through any of them.
pub type NoteRef = Rc<RefCell<Note>>;

pub struct Note {
    pub words: u32,
    pub children: Vec<NoteRef>,
}

/// A new note with `words` words and no children.
pub fn new_note(words: u32) -> NoteRef {
    // TODO: a Note with these words and an empty children Vec, inside Rc::new(RefCell::new(...))
    todo!()
}

/// Appends `child` to `parent`'s children.
pub fn add_child(parent: &NoteRef, child: NoteRef) {
    // TODO: borrow the parent mutably and push child onto its children
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_note_has_no_children() {
        let note = new_note(5);
        assert_eq!(note.borrow().words, 5);
        assert_eq!(note.borrow().children.len(), 0);
    }

    #[test]
    fn add_child_links_parent() {
        let home = new_note(1);
        add_child(&home, new_note(2));
        assert_eq!(home.borrow().children.len(), 1);
        assert_eq!(home.borrow().children[0].borrow().words, 2);
    }

    #[test]
    fn edit_through_any_handle_shows_up() {
        let home = new_note(1);
        let groceries = new_note(2);
        add_child(&home, Rc::clone(&groceries));
        groceries.borrow_mut().words = 10;
        assert_eq!(
            home.borrow().children[0].borrow().words,
            10,
            "the tree should see the edit made through `groceries`"
        );
    }
}
