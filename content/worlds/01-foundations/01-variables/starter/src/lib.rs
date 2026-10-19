/// Start at 10 points, then add a 5 point bonus.
pub fn score_with_bonus() -> i32 {
    // TODO: make `score` mutable with `mut`, then add 5 to it
    let score = 10;
    score
}

/// Length of `text` without leading/trailing spaces.
pub fn trimmed_len(text: &str) -> usize {
    // TODO: shadow `text` with `text.trim()`, then return its length
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_is_added_to_score() {
        assert_eq!(score_with_bonus(), 15, "10 + 5 bonus");
    }

    #[test]
    fn trimmed_len_ignores_spaces() {
        assert_eq!(trimmed_len("  hi  "), 2);
        assert_eq!(trimmed_len("ferris"), 6);
    }
}
