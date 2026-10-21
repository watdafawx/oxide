/// How many of an item fit in a box. Unknown items have no answer.
/// "pen" -> Some(12), "mug" -> Some(6), "lamp" -> Some(1), anything else -> None
pub fn per_box(item: &str) -> Option<u32> {
    // TODO: match item { "pen" => Some(12), ..., _ => None }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_items_have_a_box_size() {
        assert_eq!(per_box("pen"), Some(12));
        assert_eq!(per_box("mug"), Some(6));
        assert_eq!(per_box("lamp"), Some(1));
    }

    #[test]
    fn unknown_items_have_none() {
        assert_eq!(per_box("unicorn"), None, "nobody sells unicorns");
        assert_eq!(per_box(""), None);
    }
}
