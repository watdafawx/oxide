/// The store room held 10 crates; a delivery brings 5 more.
pub fn crates_after_delivery() -> u32 {
    let crates = 10;
    // TODO: add 5 to `crates` here. When Rust complains, add `mut` to the line above.
    crates
}

/// You had 48 sheets and packed one box, which uses 4.
pub fn sheets_after_packing() -> u32 {
    let sheets = 48;
    // TODO: take 4 away from `sheets`, same shape as above
    sheets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_delivery_adds_five_crates() {
        assert_eq!(crates_after_delivery(), 15, "10 crates + 5 delivered");
    }

    #[test]
    fn packing_a_box_uses_four_sheets() {
        assert_eq!(sheets_after_packing(), 44, "48 sheets - 4 for the box");
    }
}
