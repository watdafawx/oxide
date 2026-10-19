/// first_and_last(&[4, 5, 6, 7]) -> (4, 7)
pub fn first_and_last(items: &[i32]) -> (i32, i32) {
    // TODO: (items[0], items[last index])
    todo!()
}

/// swap_pair((1, 2)) -> (2, 1)
pub fn swap_pair(pair: (i32, i32)) -> (i32, i32) {
    // TODO: let (a, b) = pair; then return them flipped
    todo!()
}

/// sum_three([1, 2, 3]) -> 6
pub fn sum_three(nums: [i32; 3]) -> i32 {
    // TODO: let [a, b, c] = nums; add them
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_and_last_picks_ends() {
        assert_eq!(first_and_last(&[4, 5, 6, 7]), (4, 7));
        assert_eq!(first_and_last(&[9]), (9, 9), "one item is both first and last");
    }

    #[test]
    fn swap_pair_flips_order() {
        assert_eq!(swap_pair((1, 2)), (2, 1));
    }

    #[test]
    fn sum_three_adds_array() {
        assert_eq!(sum_three([1, 2, 3]), 6);
        assert_eq!(sum_three([-5, 5, 10]), 10);
    }
}
