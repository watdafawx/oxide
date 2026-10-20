use crate::parser::parse_line;

/// Name of the player with the highest score. Lines that don't parse are skipped.
pub fn best_player(lines: &[&str]) -> Option<String> {
    // TODO: filter_map(parse_line) over the lines, max_by_key on the score, keep the name
    todo!()
}
