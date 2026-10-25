/// The loot, a to z. [bone, string, arrow] -> [arrow, bone, string].
pub fn sorted_loot(drops: Vec<String>) -> Vec<String> {
    // TODO: make the list mutable and sort it
    drops
}

/// Each item once, a to z. [bone, arrow, bone, string, arrow] -> [arrow, bone, string].
pub fn unique_loot(drops: Vec<String>) -> Vec<String> {
    // TODO: sort, then drop the repeats
    drops
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(items: &[&str]) -> Vec<String> {
        let mut out = Vec::new();
        for item in items {
            out.push(item.to_string());
        }
        out
    }

    #[test]
    fn sorted_loot_is_a_to_z() {
        assert_eq!(sorted_loot(names(&["bone", "string", "arrow"])), names(&["arrow", "bone", "string"]));
    }

    #[test]
    fn unique_loot_has_no_repeats() {
        assert_eq!(
            unique_loot(names(&["bone", "arrow", "bone", "string", "arrow"])),
            names(&["arrow", "bone", "string"]),
            "the two bones are not neighbours until you sort"
        );
        assert_eq!(unique_loot(Vec::new()), Vec::<String>::new());
    }
}
