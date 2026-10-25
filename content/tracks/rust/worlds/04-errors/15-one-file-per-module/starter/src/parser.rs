/// The priority at the start of a line like "3|buy milk". A line without one counts as 0.
/// Given: you don't need to change it.
pub fn priority_of(line: &str) -> u32 {
    match line.split_once('|') {
        Some((priority, _)) => priority.trim().parse().unwrap_or(0),
        None => 0,
    }
}
