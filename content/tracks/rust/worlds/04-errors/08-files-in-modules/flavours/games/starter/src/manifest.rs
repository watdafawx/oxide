/// Parse "name@version" into (name, version). None if the line is malformed.
pub fn parse_mod_line(line: &str) -> Option<(String, u32)> {
    // TODO: split_once('@')?, trim both sides, parse the version with .ok()?
    todo!()
}
