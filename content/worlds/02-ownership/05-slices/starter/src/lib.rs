/// Everything before the first space; the whole text if there is no space.
pub fn first_word(text: &str) -> &str {
    // TODO: find the first space and slice up to it
    todo!()
}

/// Drops the first and last element. Empty if there are 2 or fewer.
pub fn middle(nums: &[i32]) -> &[i32] {
    // TODO: slice from index 1 to len - 1, but guard short inputs
    todo!()
}

/// The last 3 elements, or everything if there are fewer than 3.
pub fn last_three(nums: &[i32]) -> &[i32] {
    // TODO: compute the start with saturating_sub, then slice to the end
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_stops_at_space() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

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
