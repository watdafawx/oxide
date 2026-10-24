use std::cell::RefCell;

/// A note in the outliner. `open` takes &self, so the count lives in a RefCell.
pub struct Note {
    opens: RefCell<u32>,
}

impl Note {
    pub fn new() -> Note {
        Note {
            opens: RefCell::new(0),
        }
    }

    pub fn open(&self) {
        *self.opens.borrow_mut() += 1;
    }

    pub fn opens(&self) -> u32 {
        *self.opens.borrow()
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

/// Appends `tag` to the list behind the RefCell.
pub fn add_tag(tags: &RefCell<Vec<String>>, tag: &str) {
    tags.borrow_mut().push(tag.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_counts_through_shared_ref() {
        let note = Note::new();
        let shared = &note;
        shared.open();
        shared.open();
        assert_eq!(note.opens(), 2);
    }

    #[test]
    fn add_tag_pushes_onto_the_list() {
        let tags = RefCell::new(vec![String::from("home")]);
        add_tag(&tags, "urgent");
        assert_eq!(*tags.borrow(), vec!["home", "urgent"]);
    }
}
