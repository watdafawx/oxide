/// [torch, bow, arrow] -> "torch, bow, arrow"
pub fn chest_summary(items: &[&str]) -> String {
    items.join(", ")
}

/// The mod names a to z, one per line: [sodium, jei] -> "jei\nsodium"
pub fn modlist_file(mods: Vec<String>) -> String {
    let mut mods = mods;
    mods.sort();
    mods.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chest_summary_joins_with_commas() {
        assert_eq!(chest_summary(&["torch", "bow", "arrow"]), "torch, bow, arrow");
        assert_eq!(chest_summary(&["torch"]), "torch", "one item, no comma");
    }

    #[test]
    fn modlist_file_sorts_then_puts_one_per_line() {
        let mods = vec!["sodium".to_string(), "create".to_string(), "jei".to_string()];
        assert_eq!(modlist_file(mods), "create\njei\nsodium");
    }
}
