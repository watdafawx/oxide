pub mod probe;

use probe::Name;

/// The longest name (the first one wins a tie), or None for an empty slice.
pub fn longest_name(names: &[Name]) -> Option<Name> {
    // TODO: right answer, but every new leader is cloned, and each clone is counted. Keep a
    // reference to the leader while searching, and clone only the winner, once, at the end.
    let mut best = names.first()?.clone();
    for name in names {
        if name.0.len() > best.0.len() {
            best = name.clone();
        }
    }
    Some(best)
}
