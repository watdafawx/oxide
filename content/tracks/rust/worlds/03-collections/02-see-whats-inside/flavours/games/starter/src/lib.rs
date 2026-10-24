/// A debug line with every stack size: "hotbar: [1, 16, 8]".
pub fn hotbar_log(stacks: &[u32]) -> String {
    // TODO: put the whole list after "hotbar: " with format!
    String::new()
}

/// A debug line with every item name: "items: [\"sword\", \"bread\"]".
pub fn item_log(names: &[&str]) -> String {
    // TODO: the same, after "items: "
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotbar_log_shows_the_stacks() {
        assert_eq!(hotbar_log(&[1, 16, 8]), "hotbar: [1, 16, 8]");
        assert_eq!(hotbar_log(&[]), "hotbar: []", "an empty hotbar is an empty list");
    }

    #[test]
    fn item_log_shows_the_names() {
        assert_eq!(item_log(&["sword", "bread"]), "items: [\"sword\", \"bread\"]");
    }
}
