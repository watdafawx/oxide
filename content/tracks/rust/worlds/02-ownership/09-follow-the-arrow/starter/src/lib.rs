pub struct Scoreboard {
    pub scores: Vec<u32>,
}

/// Adds `n` to the caller's score.
pub fn add_points(score: &mut u32, n: u32) {
    // TODO: add n to the number that score points at
    todo!()
}

/// How many scores are bigger than `limit`.
pub fn count_over(board: &Scoreboard, limit: u32) -> usize {
    // TODO: walk the scores and count those bigger than limit
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_points_changes_the_callers_score() {
        let mut score = 10;
        add_points(&mut score, 5);
        assert_eq!(score, 15);
    }

    #[test]
    fn count_over_counts_bigger_scores() {
        let board = Scoreboard { scores: vec![4, 12, 10, 30] };
        assert_eq!(count_over(&board, 10), 2, "12 and 30 are bigger than 10; 10 is not");
        assert_eq!(count_over(&board, 100), 0);
    }
}
