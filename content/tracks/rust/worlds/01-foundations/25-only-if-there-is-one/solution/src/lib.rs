/// How many of an item fit in a box. Unknown items have no answer.
pub fn per_box(item: &str) -> Option<u32> {
    match item {
        "pen" => Some(12),
        "mug" => Some(6),
        "lamp" => Some(1),
        _ => None,
    }
}

/// "pen: 12 per box", or "no box size for unicorn" when the item is unknown.
pub fn describe_box(item: &str) -> String {
    if let Some(n) = per_box(item) {
        format!("{item}: {n} per box")
    } else {
        format!("no box size for {item}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_items_are_described() {
        assert_eq!(describe_box("pen"), "pen: 12 per box");
        assert_eq!(describe_box("lamp"), "lamp: 1 per box");
    }

    #[test]
    fn unknown_items_are_reported() {
        assert_eq!(describe_box("unicorn"), "no box size for unicorn");
    }
}
