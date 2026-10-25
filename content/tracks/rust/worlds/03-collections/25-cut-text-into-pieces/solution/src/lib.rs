/// " pen, office ,1.50" -> ["pen", "office", "1.50"]
pub fn fields(line: &str) -> Vec<&str> {
    line.split(',').map(|f| f.trim()).collect()
}

/// Every line that has text on it, trimmed: "pen\n\n  mug \n" -> ["pen", "mug"]
pub fn non_blank_lines(text: &str) -> Vec<&str> {
    text.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect()
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
