pub struct Leaderboard {
    pub levels: Vec<u32>,
}

/// Adds `n` experience to the caller's xp.
pub fn add_xp(xp: &mut u32, n: u32) {
    // TODO: add n to the number that xp points at
    todo!()
}

/// How many players are at `level` or higher.
pub fn count_at_least(board: &Leaderboard, level: u32) -> usize {
    // TODO: walk the levels and count those at `level` or higher
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_xp_changes_the_callers_xp() {
        let mut xp = 120;
        add_xp(&mut xp, 50);
        assert_eq!(xp, 170);
    }

    #[test]
    fn count_at_least_counts_high_levels() {
        let board = Leaderboard { levels: vec![5, 30, 29, 64] };
        assert_eq!(count_at_least(&board, 30), 2, "30 and 64 count; 29 does not");
        assert_eq!(count_at_least(&board, 100), 0);
    }
}
