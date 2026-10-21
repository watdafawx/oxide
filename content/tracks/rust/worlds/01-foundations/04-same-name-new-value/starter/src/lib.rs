/// How many letters a label has once the spaces around it are gone.
/// label_len("  Fragile ") -> 7
pub fn label_len(label: &str) -> usize {
    // TODO: shadow `label` with `label.trim()`, then return `label.len()`
    todo!()
}

/// Two boxes are packed, and each uses 4 sheets.
pub fn sheets_after_two_boxes(sheets: u32) -> u32 {
    // TODO: `let sheets = sheets - 4;` twice, then return `sheets`
    sheets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_is_not_counted() {
        assert_eq!(label_len("  Fragile "), 7);
        assert_eq!(label_len("Heavy"), 5);
    }

    #[test]
    fn two_boxes_use_eight_sheets() {
        assert_eq!(sheets_after_two_boxes(48), 40);
        assert_eq!(sheets_after_two_boxes(10), 2);
    }
}
