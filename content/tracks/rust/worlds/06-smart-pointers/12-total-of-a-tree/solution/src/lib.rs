use std::cell::RefCell;
use std::rc::Rc;

pub type NoteRef = Rc<RefCell<Note>>;

pub struct Note {
    pub words: u32,
    pub children: Vec<NoteRef>,
}

pub fn new_note(words: u32) -> NoteRef {
    Rc::new(RefCell::new(Note {
        words,
        children: Vec::new(),
    }))
}

pub fn add_child(parent: &NoteRef, child: NoteRef) {
    parent.borrow_mut().children.push(child);
}

/// The note's words plus the words of every note below it.
pub fn total_words(note: &NoteRef) -> u32 {
    let n = note.borrow();
    let mut sum = n.words;
    for child in &n.children {
        sum += total_words(child);
    }
    sum
}

/// How many notes: this one and every note below it.
pub fn note_count(note: &NoteRef) -> u32 {
    let n = note.borrow();
    let mut count = 1;
    for child in &n.children {
        count += note_count(child);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Home (1)
    ///   Groceries (2)
    ///     Milk (4)
    ///   Work (3)
    fn home() -> NoteRef {
        let home = new_note(1);
        let groceries = new_note(2);
        add_child(&groceries, new_note(4));
        add_child(&home, groceries);
        add_child(&home, new_note(3));
        home
    }

    #[test]
    fn total_words_adds_the_whole_tree() {
        assert_eq!(total_words(&home()), 10, "1 + 2 + 4 + 3");
        assert_eq!(total_words(&new_note(7)), 7, "a note with no children is just its own words");
    }

    #[test]
    fn note_count_counts_every_note() {
        assert_eq!(note_count(&home()), 4);
        assert_eq!(note_count(&new_note(7)), 1);
    }
}
