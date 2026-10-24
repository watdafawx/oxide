/// "12, 6, x, 1" -> [12, 6, 1]: pieces that are not numbers are skipped.
pub fn parse_counts(line: &str) -> Vec<u32> {
    // TODO: split on commas; keep each trimmed piece that parses, unwrapped
    Vec::new()
}

/// How many pieces are not numbers? "12, 6, x, 1" -> 1
pub fn count_bad(line: &str) -> usize {
    // TODO: split on commas; keep the pieces whose parse::<u32>() is an error, count them
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_counts_skips_bad_pieces() {
        assert_eq!(parse_counts("12, 6, x, 1"), vec![12, 6, 1]);
        assert_eq!(parse_counts(" 7 "), vec![7]);
        assert_eq!(parse_counts("x"), Vec::<u32>::new());
    }

    #[test]
    fn count_bad_counts_the_pieces_that_fail() {
        assert_eq!(count_bad("12, 6, x, 1"), 1);
        assert_eq!(count_bad("x, -3, 4.5"), 3, "u32 has no minus and no decimals");
        assert_eq!(count_bad("1, 2"), 0);
    }
}
