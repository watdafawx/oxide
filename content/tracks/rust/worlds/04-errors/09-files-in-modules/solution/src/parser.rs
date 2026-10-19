/// Parse "name: score" into (name, score). None if the line is malformed.
pub fn parse_line(line: &str) -> Option<(String, u32)> {
    let (name, score) = line.split_once(':')?;
    let score = score.trim().parse().ok()?;
    Some((name.trim().to_string(), score))
}
