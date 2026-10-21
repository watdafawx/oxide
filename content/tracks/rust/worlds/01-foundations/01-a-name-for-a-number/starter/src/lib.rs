/// How many crates are in the store room right now.
pub fn crates_in_store() -> u32 {
    let crates = 0; // TODO: the store room holds 12 crates
    crates
}

/// How many sheets those crates make. One crate makes four sheets.
pub fn sheets_in_store() -> u32 {
    let crates = 12;
    let sheets = 0; // TODO: four sheets per crate, so crates * 4
    sheets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_holds_twelve_crates() {
        assert_eq!(crates_in_store(), 12, "the store room holds 12 crates");
    }

    #[test]
    fn twelve_crates_make_48_sheets() {
        assert_eq!(sheets_in_store(), 48, "12 crates, four sheets each");
    }
}
