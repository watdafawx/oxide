/// Parse "name@version" into (name, version). None if the line is malformed.
pub fn parse_mod_line(line: &str) -> Option<(String, u32)> {
    let (name, version) = line.split_once('@')?;
    let version = version.trim().parse().ok()?;
    Some((name.trim().to_string(), version))
}
