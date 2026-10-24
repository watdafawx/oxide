/// The sum of every line's priority.
pub fn total_priority(lines: &[&str]) -> u32 {
    let mut total = 0;
    for line in lines {
        total += crate::parser::priority_of(line);
    }
    total
}
