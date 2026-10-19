

/// make_looting(2) returns a closure; calling it with a drop count of 3 gives 5.
pub fn make_looting(bonus: u32) -> impl Fn(u32) -> u32 {
    move |count| count + bonus
}

/// Apply `upgrade` to `level`, then apply it again to the result. upgrade_twice(|l| l * 3, 2) = 18.
pub fn upgrade_twice(upgrade: impl Fn(u32) -> u32, level: u32) -> u32 {
    upgrade(upgrade(level))
}

/// How many stack sizes make `keep` return true?
pub fn count_stacks_where(stacks: &[u32], keep: impl Fn(u32) -> bool) -> usize {
    stacks.iter().filter(|n| keep(**n)).count()
}

/// Keep stack sizes strictly greater than `limit`: ([1, 5, 64], 4) -> [5, 64]
pub fn stacks_over(stacks: &[u32], limit: u32) -> Vec<u32> {
    stacks.iter().copied().filter(|n| *n > limit).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_adder_captures_n() {
        let looting_ii = make_looting(2);
        assert_eq!(looting_ii(3), 5);
        assert_eq!(make_looting(0)(7), 7);
    }

    #[test]
    fn apply_twice_calls_f_twice() {
        assert_eq!(upgrade_twice(|n| n * 3, 2), 18);
        assert_eq!(upgrade_twice(|n| n + 1, 0), 2);
    }

    #[test]
    fn count_matching_uses_predicate() {
        assert_eq!(count_stacks_where(&[1, 2, 3, 4, 5], |n| n > 2), 3);
        assert_eq!(count_stacks_where(&[1, 2, 3, 4, 5], |n| n % 2 == 0), 2);
        assert_eq!(count_stacks_where(&[], |_| true), 0);
    }

    #[test]
    fn above_limit_captures_limit() {
        assert_eq!(stacks_over(&[1, 5, 64], 4), vec![5, 64]);
        assert_eq!(stacks_over(&[1, 5, 64], 64), Vec::<u32>::new());
        assert_eq!(stacks_over(&[1, 5, 64], 0), vec![1, 5, 64]);
    }
}
