use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type NoteRef = Rc<RefCell<Note>>;

/// A note owns its children and points up at its parent.
pub struct Note {
    pub title: String,
    pub parent: Weak<RefCell<Note>>,
    pub children: Vec<NoteRef>,
}

pub fn new_root(title: &str) -> NoteRef {
    Rc::new(RefCell::new(Note {
        title: title.to_string(),
        parent: Weak::new(),
        children: Vec::new(),
    }))
}

pub fn add_child(parent: &NoteRef, title: &str) -> NoteRef {
    let child = Rc::new(RefCell::new(Note {
        title: title.to_string(),
        parent: Rc::downgrade(parent),
        children: Vec::new(),
    }));
    parent.borrow_mut().children.push(Rc::clone(&child));
    child
}

/// Titles from the root down to `note`.
pub fn breadcrumb(note: &NoteRef) -> Vec<String> {
    let mut crumbs = Vec::new();
    let mut current = Some(Rc::clone(note));
    while let Some(n) = current {
        crumbs.push(n.borrow().title.clone());
        current = n.borrow().parent.upgrade();
    }
    crumbs.reverse();
    crumbs
}

/// True when some note above `note` (not the note itself) has this title.
pub fn has_ancestor(note: &NoteRef, title: &str) -> bool {
    let mut current = note.borrow().parent.upgrade();
    while let Some(n) = current {
        if n.borrow().title == title {
            return true;
        }
        current = n.borrow().parent.upgrade();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn breadcrumb_lists_root_down_to_note() {
        let home = new_root("Home");
        let groceries = add_child(&home, "Groceries");
        let milk = add_child(&groceries, "Milk");
        assert_eq!(breadcrumb(&milk), vec!["Home", "Groceries", "Milk"]);
        assert_eq!(breadcrumb(&home), vec!["Home"], "the root's breadcrumb is just itself");
    }

    #[test]
    fn has_ancestor_finds_titles_above() {
        let home = new_root("Home");
        let groceries = add_child(&home, "Groceries");
        let milk = add_child(&groceries, "Milk");
        let work = add_child(&home, "Work");
        assert!(has_ancestor(&milk, "Home"));
        assert!(has_ancestor(&milk, "Groceries"));
        assert!(!has_ancestor(&milk, "Milk"), "a note is not its own ancestor");
        assert!(!has_ancestor(&work, "Groceries"), "Groceries is a sibling, not above Work");
        assert!(!has_ancestor(&home, "Home"), "the root has nothing above it");
    }
}
