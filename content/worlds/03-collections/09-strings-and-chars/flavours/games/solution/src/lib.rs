

/// "64, 16, x, 8" -> [64, 16, 8]. Pieces that are not numbers are skipped.
pub fn parse_counts(line: &str) -> Vec<u32> {
    line.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

/// "minecraft:diamond_sword" -> "Diamond Sword". Drop the namespace, split on '_', capitalise each word.
pub fn display_name(id: &str) -> String {
    let name = id.rsplit(':').next().unwrap_or(id);
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// ["torch", "bow", "arrow"] -> "arrow, bow, torch"
pub fn chest_summary(items: &[&str]) -> String {
    let mut list = items.to_vec();
    list.sort();
    list.join(", ")
}

/// Name padded to 12 characters on the left, count right-aligned in 4: "diamond       12"
pub fn inventory_row(name: &str, count: u32) -> String {
    format!("{name:<12}{count:>4}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_splits_trims_parses() {
        assert_eq!(parse_counts("64, 16, x, 8"), vec![64, 16, 8]);
        assert_eq!(parse_counts(" 7 "), vec![7]);
        assert_eq!(parse_counts(""), Vec::<u32>::new());
    }

    #[test]
    fn count_vowels_walks_chars() {
        assert_eq!(display_name("minecraft:diamond_sword"), "Diamond Sword");
        assert_eq!(display_name("minecraft:oak_log"), "Oak Log");
        assert_eq!(display_name("stone"), "Stone", "no namespace is fine");
    }

    #[test]
    fn join_sorted_joins_with_commas() {
        assert_eq!(chest_summary(&["torch", "bow", "arrow"]), "arrow, bow, torch");
        assert_eq!(chest_summary(&[]), "");
    }

    #[test]
    fn table_row_pads_columns() {
        assert_eq!(inventory_row("diamond", 12), "diamond       12");
        assert_eq!(inventory_row("torch", 3), "torch          3");
    }
}
