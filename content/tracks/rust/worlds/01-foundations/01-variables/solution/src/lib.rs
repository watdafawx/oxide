/// Start at 10 points, then add a 5 point bonus.
pub fn score_with_bonus() -> i32 {
    let mut score = 10;
    score += 5;
    score
}

/// Length of `text` without leading/trailing spaces.
pub fn trimmed_len(text: &str) -> usize {
    let text = text.trim();
    text.len()
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
