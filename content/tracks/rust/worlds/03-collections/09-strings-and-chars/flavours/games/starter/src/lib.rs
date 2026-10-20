

/// "64, 16, x, 8" -> [64, 16, 8]. Pieces that are not numbers are skipped.
pub fn parse_counts(line: &str) -> Vec<u32> {
    // TODO: split on commas, trim each piece, keep the ones that parse as numbers, drop the rest
    todo!()
}

/// "minecraft:diamond_sword" -> "Diamond Sword". Drop the namespace, split on '_', capitalise each word.
pub fn display_name(id: &str) -> String {
    // TODO: cut off the namespace (everything up to the last ':')
    // TODO: split the name into words on '_' and capitalise the first letter of each word (`to_uppercase()` on a char gives an iterator, see the lesson)
    // TODO: join the capitalised words with single spaces
    todo!()
}

/// ["torch", "bow", "arrow"] -> "arrow, bow, torch"
pub fn chest_summary(items: &[&str]) -> String {
    // TODO: sort a copy of the items and join them with ", "
    todo!()
}

/// Name padded to 12 characters on the left, count right-aligned in 4: "diamond       12"
pub fn inventory_row(name: &str, count: u32) -> String {
    // TODO: pad the name to width 12 (left) and the count to width 4 (right)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_counts_skips_bad_entries() {
        assert_eq!(parse_counts("64, 16, x, 8"), vec![64, 16, 8]);
        assert_eq!(parse_counts(" 7 "), vec![7]);
        assert_eq!(parse_counts(""), Vec::<u32>::new());
    }

    #[test]
    fn display_name_title_cases_item_ids() {
        assert_eq!(display_name("minecraft:diamond_sword"), "Diamond Sword");
        assert_eq!(display_name("minecraft:oak_log"), "Oak Log");
        assert_eq!(display_name("stone"), "Stone", "no namespace is fine");
    }

    #[test]
    fn chest_summary_sorts_and_joins() {
        assert_eq!(chest_summary(&["torch", "bow", "arrow"]), "arrow, bow, torch");
        assert_eq!(chest_summary(&[]), "");
    }

    #[test]
    fn inventory_row_aligns_name_and_count() {
        assert_eq!(inventory_row("diamond", 12), "diamond       12");
        assert_eq!(inventory_row("torch", 3), "torch          3");
    }
}
