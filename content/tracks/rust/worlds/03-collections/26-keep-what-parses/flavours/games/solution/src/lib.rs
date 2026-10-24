/// "64, 16, x, 8" -> [64, 16, 8]: pieces that are not numbers are skipped.
pub fn parse_stacks(line: &str) -> Vec<u32> {
    line.split(',').filter_map(|s| s.trim().parse().ok()).collect()
}

/// How many pieces are not numbers? "64, 16, x, 8" -> 1
pub fn count_bad(line: &str) -> usize {
    line.split(',').filter(|s| s.trim().parse::<u32>().is_err()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stacks_skips_bad_pieces() {
        assert_eq!(parse_stacks("64, 16, x, 8"), vec![64, 16, 8]);
        assert_eq!(parse_stacks(" 7 "), vec![7]);
        assert_eq!(parse_stacks("x"), Vec::<u32>::new());
    }

    #[test]
    fn count_bad_counts_the_pieces_that_fail() {
        assert_eq!(count_bad("64, 16, x, 8"), 1);
        assert_eq!(count_bad("x, -3, 4.5"), 3, "a stack has no minus and no decimals");
        assert_eq!(count_bad("1, 2"), 0);
    }
}
