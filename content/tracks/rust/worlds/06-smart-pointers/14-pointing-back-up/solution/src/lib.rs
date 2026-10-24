use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// A note owns its children (Rc) and points up at its parent (Weak).
pub struct Note {
    pub title: String,
    pub parent: Weak<Note>,
    pub children: RefCell<Vec<Rc<Note>>>,
}

/// A note with no parent.
pub fn new_root(title: &str) -> Rc<Note> {
    Rc::new(Note {
        title: title.to_string(),
        parent: Weak::new(),
        children: RefCell::new(Vec::new()),
    })
}

/// A new note under `parent`. The parent owns it; it only points back.
pub fn new_child(parent: &Rc<Note>, title: &str) -> Rc<Note> {
    let child = Rc::new(Note {
        title: title.to_string(),
        parent: Rc::downgrade(parent),
        children: RefCell::new(Vec::new()),
    });
    parent.children.borrow_mut().push(Rc::clone(&child));
    child
}

/// The parent's title, or None for a root or a parent that is gone.
pub fn parent_title(note: &Note) -> Option<String> {
    match note.parent.upgrade() {
        Some(parent) => Some(parent.title.clone()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_child_links_both_ways() {
        let home = new_root("Home");
        assert_eq!(parent_title(&home), None, "a root has no parent");
        let groceries = new_child(&home, "Groceries");
        assert_eq!(home.children.borrow().len(), 1);
        assert_eq!(home.children.borrow()[0].title, "Groceries");
        assert_eq!(parent_title(&groceries), Some(String::from("Home")));
    }

    #[test]
    fn parent_link_is_weak() {
        let home = new_root("Home");
        let groceries = new_child(&home, "Groceries");
        assert_eq!(Rc::strong_count(&home), 1, "a child must not own its parent");
        assert_eq!(Rc::weak_count(&home), 1);
        assert_eq!(Rc::strong_count(&groceries), 2, "your handle + the parent's list");
    }

    #[test]
    fn parent_title_is_none_after_parent_dropped() {
        let home = new_root("Home");
        let groceries = new_child(&home, "Groceries");
        drop(home);
        assert_eq!(parent_title(&groceries), None, "Home is gone, upgrade() must give None");
        assert_eq!(Rc::strong_count(&groceries), 1, "Home's list let go of its handle");
    }
}
