/// A filed note: it owns its text and remembers how long it is.
#[derive(Debug, PartialEq)]
pub struct Note {
    pub text: String,
    pub letters: usize,
}

/// Files the text as a note.
pub fn file_note(words: String) -> Note {
    // TODO: this does not build: `words` moves into the note, then `words.len()` uses it again.
    // Work out the length before the text moves.
    Note { text: words, letters: words.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_note_keeps_text_and_length() {
        let note = file_note(String::from("buy milk"));
        assert_eq!(note.text, "buy milk");
        assert_eq!(note.letters, 8, "\"buy milk\" has 8 letters, counting the space");
    }
}
