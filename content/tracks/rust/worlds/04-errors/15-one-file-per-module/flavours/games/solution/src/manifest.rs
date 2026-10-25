/// The version in a mod line like "sodium@5". A line without one counts as 0.
/// Given: you don't need to change it.
pub fn version_of(line: &str) -> u32 {
    match line.split_once('@') {
        Some((_, version)) => version.trim().parse().unwrap_or(0),
        None => 0,
    }
}
