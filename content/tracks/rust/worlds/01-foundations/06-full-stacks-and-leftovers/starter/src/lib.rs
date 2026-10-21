/// A pallet holds 64 sheets. full_pallets(150) -> 2
pub fn full_pallets(sheets: u32) -> u32 {
    0 // TODO: how many times does 64 fit into `sheets`?
}

/// Sheets that do not fill a pallet. loose_sheets(150) -> 22
pub fn loose_sheets(sheets: u32) -> u32 {
    0 // TODO: what is left after the full pallets?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_pallets_count_whole_pallets() {
        assert_eq!(full_pallets(150), 2);
        assert_eq!(full_pallets(63), 0);
        assert_eq!(full_pallets(64), 1);
    }

    #[test]
    fn loose_sheets_are_the_remainder() {
        assert_eq!(loose_sheets(150), 22);
        assert_eq!(loose_sheets(63), 63);
        assert_eq!(loose_sheets(128), 0, "an exact fit leaves nothing");
    }
}
