/// Start with 100 and push 80, then 60. Returns [100, 80, 60].
pub fn podium() -> Vec<u32> {
    let mut scores = vec![100];
    // TODO: push 80 and 60 onto `scores`, then return it
    todo!()
}

/// The third item, or None if there are fewer than three.
pub fn third(items: &[i32]) -> Option<i32> {
    // TODO: read index 2 without panicking; return the number itself, not a reference to it
    todo!()
}

/// Adds up every number. An empty slice gives 0.
pub fn sum_all(numbers: &[i32]) -> i32 {
    // TODO: add every number in a `for` loop and return the total
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn podium_pushes_two_more() {
        assert_eq!(podium(), vec![100, 80, 60]);
    }

    #[test]
    fn third_uses_get() {
        assert_eq!(third(&[1, 2, 3, 4]), Some(3));
        assert_eq!(third(&[1, 2]), None, "two items means no third");
        assert_eq!(third(&[]), None);
    }

    #[test]
    fn sum_all_loops_over_the_vec() {
        assert_eq!(sum_all(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_all(&[-5, 5, 7]), 7);
        assert_eq!(sum_all(&[]), 0);
    }
}
