/// One line of the stock sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct StockLine {
    pub name: String,
    pub qty: u32,
}

/// The longest name, for sizing the shelf labels. None for an empty list.
pub fn longest_name(names: &[String]) -> Option<&String> {
    // TODO: pick the name whose length is biggest
    None
}

/// The line with the fewest units left. None for an empty sheet.
pub fn fewest_left(lines: &[StockLine]) -> Option<&StockLine> {
    // TODO: pick the line whose qty is smallest
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(name: &str, qty: u32) -> StockLine {
        StockLine { name: name.to_string(), qty }
    }

    #[test]
    fn longest_name_picks_by_length() {
        let names = vec!["pen".to_string(), "notebook".to_string(), "mug".to_string()];
        assert_eq!(longest_name(&names), Some(&names[1]));
        assert_eq!(longest_name(&[]), None);
    }

    #[test]
    fn fewest_left_picks_the_smallest_qty() {
        let lines = vec![line("pen", 12), line("lamp", 1), line("mug", 6)];
        assert_eq!(fewest_left(&lines), Some(&lines[1]));
        assert_eq!(fewest_left(&[]), None);
    }
}
