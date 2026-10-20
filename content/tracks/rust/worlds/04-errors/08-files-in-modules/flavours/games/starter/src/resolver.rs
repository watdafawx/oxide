use crate::manifest::parse_mod_line;

/// Name of the mod with the highest version. Lines that don't parse are skipped.
pub fn newest_mod(lines: &[&str]) -> Option<String> {
    // TODO: parse every line, skip the bad ones, pick the highest version and return its name (the lines are `&&str` here: wrap the call in a closure)
    todo!()
}
