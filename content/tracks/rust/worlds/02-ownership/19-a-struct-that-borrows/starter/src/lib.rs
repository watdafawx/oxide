/// A saved note: owns its title.
pub struct Note {
    pub title: String,
    pub pages: u32,
}

/// A quick view of a note for the summary screen. It borrows the title instead of copying it.
// TODO: this does not build: say how long the borrowed title lives.
// Give the struct a lifetime `'a` and use it on the field.
#[derive(Debug, PartialEq)]
pub struct NoteView {
    pub title: &str,
    pub pages: u32,
}

impl Note {
    /// A view of this note that borrows its title.
    pub fn view(&self) -> NoteView<'_> {
        // TODO: build a NoteView whose title borrows this note's title
        todo!()
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
