pub mod probe;

use probe::Name;

/// The longest name (the first one wins a tie), or None for an empty slice.
pub fn longest_name(names: &[Name]) -> Option<Name> {
    let mut best = names.first()?;
    for name in names {
        if name.0.len() > best.0.len() {
            best = name;
        }
    }
    Some(best.clone())
}
