/// " pen, office ,1.50" -> ["pen", "office", "1.50"]
pub fn fields(line: &str) -> Vec<&str> {
    // TODO: split the line on commas, trim every piece, collect
    Vec::new()
}

/// Every line that has text on it, trimmed: "pen\n\n  mug \n" -> ["pen", "mug"]
pub fn non_blank_lines(text: &str) -> Vec<&str> {
    // TODO: walk the lines, trim each, keep the ones that are not empty (`.is_empty()`), collect
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_split_and_trim() {
        assert_eq!(fields(" pen, office ,1.50"), vec!["pen", "office", "1.50"]);
        assert_eq!(fields("lamp"), vec!["lamp"], "no comma, one field");
    }

    #[test]
    fn non_blank_lines_skip_empty_ones() {
        assert_eq!(non_blank_lines("pen\n\n  mug \n"), vec!["pen", "mug"]);
        assert_eq!(non_blank_lines("\n   \n"), Vec::<&str>::new());
    }
}
