/// A saved note.
pub struct Note {
    pub title: String,
    pub pages: u32,
}

impl Note {
    pub fn new(title: &str, page_count: u32) -> Note {
        Note { title: title.to_string(), pages: page_count }
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
