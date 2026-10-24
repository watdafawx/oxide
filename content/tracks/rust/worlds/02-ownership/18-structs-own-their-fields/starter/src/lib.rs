/// A saved note.
pub struct Note {
    // TODO: this does not build: a field can't just borrow text. Give the note its own String.
    pub title: &str,
    pub pages: u32,
}

impl Note {
    pub fn new(title: &str, page_count: u32) -> Note {
        // TODO: once `title` is a String, store an owned copy of the text here
        Note { title: title, pages: page_count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_outlives_the_typed_text() {
        let note = {
            let typed = String::from("shopping");
            Note::new(&typed, 2)
        };
        assert_eq!(note.title, "shopping");
        assert_eq!(note.pages, 2);
    }
}
