/// How many of an item fit in a box. Unknown items have no answer.
pub fn per_box(item: &str) -> Option<u32> {
    match item {
        "pen" => Some(12),
        "mug" => Some(6),
        "lamp" => Some(1),
        _ => None,
    }
}

/// per_box, but 0 when the item is unknown.
pub fn per_box_or_zero(item: &str) -> u32 {
    per_box(item).unwrap_or(0)
}

/// How many items are in `boxes` boxes of `item`. Unknown items count as 0.
/// items_in_boxes("pen", 3) -> 36
pub fn items_in_boxes(item: &str, boxes: u32) -> u32 {
    per_box(item).unwrap_or(0) * boxes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_items_give_their_count() {
        assert_eq!(per_box_or_zero("pen"), 12);
        assert_eq!(per_box_or_zero("lamp"), 1);
        assert_eq!(per_box_or_zero("unicorn"), 0);
    }

    #[test]
    fn boxes_multiply_the_count() {
        assert_eq!(items_in_boxes("pen", 3), 36);
        assert_eq!(items_in_boxes("mug", 2), 12);
        assert_eq!(items_in_boxes("unicorn", 5), 0);
    }
}
