/// Parse "name@version" into (name, version). None if the line is malformed.
pub fn parse_mod_line(line: &str) -> Option<(String, u32)> {
    // TODO: split the line at '@', trim both sides, parse the version; any failure gives None
    todo!()
}
