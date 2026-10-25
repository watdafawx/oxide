/// A saved note: owns its title.
pub struct Note {
    pub title: String,
    pub pages: u32,
}

/// A quick view of a note for the summary screen. It borrows the title instead of copying it.
#[derive(Debug, PartialEq)]
pub struct NoteView<'a> {
    pub title: &'a str,
    pub pages: u32,
}

impl Note {
    /// A view of this note that borrows its title.
    pub fn view(&self) -> NoteView<'_> {
        NoteView { title: &self.title, pages: self.pages }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_borrows_the_title() {
        let note = Note { title: String::from("shopping"), pages: 2 };
        let view = note.view();
        assert_eq!(view, NoteView { title: "shopping", pages: 2 });
        assert_eq!(view.title.as_ptr(), note.title.as_ptr(), "the view must borrow the title, not copy it");
    }
}
