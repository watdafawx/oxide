/// `"[x] sodium"` when enabled, `"[ ] sodium"` when not.
pub fn card_line(name: &str, enabled: bool) -> String {
    let mark = if enabled { "[x]" } else { "[ ]" };
    format!("{mark} {name}")
}

/// `"2 of 3 mods enabled"` for `[("sodium", true), ("iris", false), ("lithium", true)]`.
pub fn pack_summary(mods: &[(&str, bool)]) -> String {
    let on = mods.iter().filter(|(_, enabled)| *enabled).count();
    format!("{on} of {} mods enabled", mods.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_line_shows_a_checkbox() {
        assert_eq!(card_line("sodium", true), "[x] sodium");
        assert_eq!(card_line("iris", false), "[ ] iris");
    }

    #[test]
    fn pack_summary_counts_enabled_mods() {
        let mods = [("sodium", true), ("iris", false), ("lithium", true)];
        assert_eq!(pack_summary(&mods), "2 of 3 mods enabled");
        assert_eq!(pack_summary(&[]), "0 of 0 mods enabled");
    }
}
