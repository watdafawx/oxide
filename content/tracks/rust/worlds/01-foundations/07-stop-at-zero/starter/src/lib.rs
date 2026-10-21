/// Take `amount` sheets from a pile of `pile`. The pile never goes below zero.
/// take_sheets(10, 3) -> 7, take_sheets(3, 5) -> 0
pub fn take_sheets(pile: u32, amount: u32) -> u32 {
    // TODO: a plain `-` panics when amount is bigger than pile. Use `.saturating_sub` instead.
    pile - amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taking_some_leaves_the_rest() {
        assert_eq!(take_sheets(10, 3), 7);
        assert_eq!(take_sheets(5, 5), 0);
    }

    #[test]
    fn taking_too_many_stops_at_zero() {
        assert_eq!(take_sheets(3, 5), 0, "the pile cannot go below zero");
        assert_eq!(take_sheets(0, 1), 0);
    }
}
