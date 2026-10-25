/// [pen, mug, lamp] -> "pen, mug, lamp"
pub fn shelf_label(names: &[&str]) -> String {
    // TODO: glue the names with ", " in between
    String::new()
}

/// The names a to z, one per line: [pen, lamp] -> "lamp\npen"
pub fn order_sheet(names: Vec<String>) -> String {
    // TODO: make the list mutable, sort it, and join it with a line break
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shelf_label_joins_with_commas() {
        assert_eq!(shelf_label(&["pen", "mug", "lamp"]), "pen, mug, lamp");
        assert_eq!(shelf_label(&["pen"]), "pen", "one item, no comma");
    }

    #[test]
    fn order_sheet_sorts_then_puts_one_per_line() {
        let names = vec!["pen".to_string(), "lamp".to_string(), "mug".to_string()];
        assert_eq!(order_sheet(names), "lamp\nmug\npen");
    }
}
