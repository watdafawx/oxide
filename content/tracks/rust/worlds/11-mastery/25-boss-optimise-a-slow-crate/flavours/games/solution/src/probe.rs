//! The comparison meter. Nothing to edit here.
//!
//! `BlockName` is a `String` whose `==`, `<`/`cmp` and `hash` each count as one operation.
//! Read the running total with [`ops`].
use std::cell::Cell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

thread_local! {
    static OPS: Cell<usize> = const { Cell::new(0) };
}

fn tick() {
    OPS.with(|c| c.set(c.get() + 1));
}

/// How many `BlockName` comparisons and hashes happened on this thread so far.
pub fn ops() -> usize {
    OPS.with(Cell::get)
}

#[derive(Debug, Clone)]
pub struct BlockName(pub String);

impl PartialEq for BlockName {
    fn eq(&self, other: &Self) -> bool {
        tick();
        self.0 == other.0
    }
}

impl Eq for BlockName {}

impl PartialOrd for BlockName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BlockName {
    fn cmp(&self, other: &Self) -> Ordering {
        tick();
        self.0.cmp(&other.0)
    }
}

impl Hash for BlockName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        tick();
        self.0.hash(state);
    }
}
