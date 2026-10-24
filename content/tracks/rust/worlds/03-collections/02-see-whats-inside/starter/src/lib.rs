/// A log line with every count: "stock: [12, 6, 1]".
pub fn stock_log(counts: &[u32]) -> String {
    // TODO: put the whole list after "stock: " with format!
    String::new()
}

/// A log line with every product name: "shelf: [\"pen\", \"mug\"]".
pub fn shelf_log(names: &[&str]) -> String {
    // TODO: the same, after "shelf: "
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_log_shows_the_counts() {
        assert_eq!(stock_log(&[12, 6, 1]), "stock: [12, 6, 1]");
        assert_eq!(stock_log(&[]), "stock: []", "an empty shelf is an empty list");
    }

    #[test]
    fn shelf_log_shows_the_names() {
        assert_eq!(shelf_log(&["pen", "mug"]), "shelf: [\"pen\", \"mug\"]");
    }
}
