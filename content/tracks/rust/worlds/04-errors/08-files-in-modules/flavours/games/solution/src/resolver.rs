use crate::manifest::parse_mod_line;

/// Name of the mod with the highest version. Lines that don't parse are skipped.
pub fn newest_mod(lines: &[&str]) -> Option<String> {
    lines
        .iter()
        .filter_map(|l| parse_mod_line(l))
        .max_by_key(|(_, version)| *version)
        .map(|(name, _)| name)
}
