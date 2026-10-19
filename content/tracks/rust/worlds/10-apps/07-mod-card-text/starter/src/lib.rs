/// `"[x] sodium"` when enabled, `"[ ] sodium"` when not.
pub fn mod_line(name: &str, enabled: bool) -> String {
    // TODO: pick "[x]" or "[ ]" and join it with the name
    todo!()
}

/// `"2 of 3 mods enabled"` for `[("sodium", true), ("iris", false), ("lithium", true)]`.
pub fn mods_summary(mods: &[(&str, bool)]) -> String {
    // TODO: count the enabled ones, then format "<on> of <total> mods enabled"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_line_shows_a_checkbox() {
        assert_eq!(mod_line("sodium", true), "[x] sodium");
        assert_eq!(mod_line("iris", false), "[ ] iris");
    }

    #[test]
    fn summary_counts_enabled_mods() {
        let mods = [("sodium", true), ("iris", false), ("lithium", true)];
        assert_eq!(mods_summary(&mods), "2 of 3 mods enabled");
        assert_eq!(mods_summary(&[]), "0 of 0 mods enabled");
    }
}
