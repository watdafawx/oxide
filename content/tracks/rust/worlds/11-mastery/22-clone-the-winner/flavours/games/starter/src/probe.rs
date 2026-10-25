//! The clone meter. Nothing to edit here.
use std::cell::Cell;

thread_local! {
    static CLONES: Cell<usize> = const { Cell::new(0) };
}

/// A name whose `clone` is counted. Read the total with [`clones`].
#[derive(Debug, PartialEq, Eq)]
pub struct Name(pub String);

impl Clone for Name {
    fn clone(&self) -> Self {
        CLONES.with(|c| c.set(c.get() + 1));
        Name(self.0.clone())
    }
}

/// How many `Name`s were cloned on this thread so far.
pub fn clones() -> usize {
    CLONES.with(Cell::get)
}
