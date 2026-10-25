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

/// True when `a` and `b` are handles to the very same note.
pub fn is_same(a: &NoteRef, b: &NoteRef) -> bool {
    // TODO: do a and b point at the same note on the heap?
    todo!()
}

/// Takes `note` out of its parent's children. A note without a parent is left alone.
pub fn detach(note: &NoteRef) {
    // TODO: get the parent in its own statement; if there is one,
    // retain every child of it that is not this exact note
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn titles(note: &NoteRef) -> Vec<String> {
        let mut out = Vec::new();
        for child in &note.borrow().children {
            out.push(child.borrow().title.clone());
        }
        out
    }

    #[test]
    fn is_same_tells_look_alikes_apart() {
        let list = new_root("Groceries");
        let milk = add_child(&list, "Milk");
        let more_milk = add_child(&list, "Milk");
        assert!(is_same(&milk, &Rc::clone(&milk)), "a clone is the same note");
        assert!(!is_same(&milk, &more_milk), "same title, different note");
    }

    #[test]
    fn detach_removes_only_that_note() {
        let list = new_root("Groceries");
        let milk = add_child(&list, "Milk");
        let more_milk = add_child(&list, "Milk");
        add_child(&list, "Bread");
        detach(&more_milk);
        assert_eq!(titles(&list), vec!["Milk", "Bread"]);
        assert!(Rc::ptr_eq(&list.borrow().children[0], &milk), "the first Milk must stay");
        detach(&list);
        assert_eq!(titles(&list), vec!["Milk", "Bread"], "the root has no parent, nothing changes");
    }
}
