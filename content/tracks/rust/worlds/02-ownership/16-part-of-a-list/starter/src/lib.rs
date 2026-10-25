/// Drops the first and the last item. Empty if there are 2 or fewer.
pub fn middle(nums: &[i32]) -> &[i32] {
    // TODO: borrow everything except the two ends, but guard short lists
    todo!()
}

/// The last 3 items, or everything if there are fewer than 3.
pub fn last_three(nums: &[i32]) -> &[i32] {
    // TODO: work out where the last three start (never below 0), then borrow to the end
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_drops_both_ends() {
        assert_eq!(middle(&[1, 2, 3, 4]), &[2, 3]);
        assert_eq!(middle(&[1, 2]), &[] as &[i32]);
        assert_eq!(middle(&[]), &[] as &[i32]);
    }

    #[test]
    fn last_three_takes_the_tail() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(last_three(&v), &[3, 4, 5]);
        assert_eq!(last_three(&[8, 9]), &[8, 9]);
    }
}
