pub struct Scoreboard {
    pub scores: Vec<u32>,
}

/// Adds `n` to the caller's score.
pub fn add_points(score: &mut u32, n: u32) {
    *score += n;
}

/// How many scores are bigger than `limit`.
pub fn count_over(board: &Scoreboard, limit: u32) -> usize {
    let mut count = 0;
    for s in &board.scores {
        if *s > limit {
            count += 1;
        }
    }
    count
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
