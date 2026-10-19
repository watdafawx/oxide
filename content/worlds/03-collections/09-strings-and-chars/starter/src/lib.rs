/// "1, 2, x, 4" -> [1, 2, 4]. Pieces that are not numbers are skipped.
pub fn parse_numbers(line: &str) -> Vec<i32> {
    // TODO: split(','), then filter_map(|s| s.trim().parse().ok()), then collect
    todo!()
}

/// How many of a, e, i, o, u (either case) are in the text?
pub fn count_vowels(text: &str) -> usize {
    // TODO: text.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
    todo!()
}

/// ["pear", "apple", "fig"] -> "apple, fig, pear"
pub fn join_sorted(words: &[&str]) -> String {
    // TODO: copy with words.to_vec(), sort it, then .join(", ")
    todo!()
}

/// Name padded to 10 characters on the left, qty right-aligned in 5: "bolts        12"
pub fn table_row(name: &str, qty: u32) -> String {
    // TODO: format!("{name:<10}{qty:>5}")
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_splits_trims_parses() {
        assert_eq!(parse_numbers("1, 2, x, 4"), vec![1, 2, 4]);
        assert_eq!(parse_numbers(" 7 "), vec![7]);
        assert_eq!(parse_numbers(""), Vec::<i32>::new());
    }

    #[test]
    fn count_vowels_walks_chars() {
        assert_eq!(count_vowels("Hello World"), 3);
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
        assert_eq!(count_vowels("rhythm"), 0);
    }

    #[test]
    fn join_sorted_joins_with_commas() {
        assert_eq!(join_sorted(&["pear", "apple", "fig"]), "apple, fig, pear");
        assert_eq!(join_sorted(&[]), "");
    }

    #[test]
    fn table_row_pads_columns() {
        assert_eq!(table_row("bolts", 12), "bolts        12");
        assert_eq!(table_row("nuts", 3), "nuts          3");
    }
}
