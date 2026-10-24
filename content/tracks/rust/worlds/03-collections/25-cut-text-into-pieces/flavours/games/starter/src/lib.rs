/// " Steve, diamond ,3" -> ["Steve", "diamond", "3"]
pub fn command_args(line: &str) -> Vec<&str> {
    // TODO: split the line on commas, trim every piece, collect
    Vec::new()
}

/// Every line of a mod list that has text on it, trimmed: "sodium\n\n  jei \n" -> ["sodium", "jei"]
pub fn mod_lines(text: &str) -> Vec<&str> {
    // TODO: walk the lines, trim each, keep the ones that are not empty (`.is_empty()`), collect
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_args_split_and_trim() {
        assert_eq!(command_args(" Steve, diamond ,3"), vec!["Steve", "diamond", "3"]);
        assert_eq!(command_args("spawn"), vec!["spawn"], "no comma, one argument");
    }

    #[test]
    fn mod_lines_skip_empty_ones() {
        assert_eq!(mod_lines("sodium\n\n  jei \n"), vec!["sodium", "jei"]);
        assert_eq!(mod_lines("\n   \n"), Vec::<&str>::new());
    }
}
