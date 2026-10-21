/// One crate makes four sheets.
pub fn sheets(crates: u32) -> u32 {
    crates * 4
}

/// One sheet makes two boxes.
pub fn boxes(sheets: u32) -> u32 {
    sheets * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheets_for_any_crate_count() {
        assert_eq!(sheets(12), 48);
        assert_eq!(sheets(7), 28);
        assert_eq!(sheets(0), 0);
    }

    #[test]
    fn boxes_for_any_sheet_count() {
        assert_eq!(boxes(48), 96);
        assert_eq!(boxes(1), 2);
    }
}
