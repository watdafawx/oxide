use crate::parser::parse_line;

/// Name of the player with the highest score. Lines that don't parse are skipped.
pub fn best_player(lines: &[&str]) -> Option<String> {
    lines
        .iter()
        .filter_map(|l| parse_line(l))
        .max_by_key(|(_, score)| *score)
        .map(|(name, _)| name)
}
