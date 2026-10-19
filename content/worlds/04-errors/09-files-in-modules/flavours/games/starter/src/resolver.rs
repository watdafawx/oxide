use crate::manifest::parse_mod_line;

/// Name of the mod with the highest version. Lines that don't parse are skipped.
pub fn newest_mod(lines: &[&str]) -> Option<String> {
    // TODO: filter_map(parse_mod_line) over the lines, max_by_key on the version, keep the name
    todo!()
}
