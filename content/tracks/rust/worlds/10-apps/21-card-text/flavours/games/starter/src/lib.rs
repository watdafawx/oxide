/// One line of the mod list: `[x] sodium` when enabled, `[ ] sodium` when not.
pub fn card_line(name: &str, enabled: bool) -> String {
    // TODO: pick the mark from `enabled` and put it in front of the name, with a space between
    todo!()
}

/// The header: `2 of 3 mods enabled` for a list of (name, enabled) pairs.
pub fn pack_summary(mods: &[(&str, bool)]) -> String {
    // TODO: count the pairs whose enabled is true, then build the sentence
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_line_shows_a_checkbox() {
        assert_eq!(card_line("sodium", true), "[x] sodium");
        assert_eq!(card_line("sodium", false), "[ ] sodium");
    }

    #[test]
    fn pack_summary_counts_enabled_mods() {
        let mods = [("sodium", true), ("optifine", false), ("iris", true)];
        assert_eq!(pack_summary(&mods), "2 of 3 mods enabled");
        assert_eq!(pack_summary(&[]), "0 of 0 mods enabled");
    }
}
