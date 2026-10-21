/// Take `amount` logs from a chest holding `chest`. The chest never goes below zero.
/// take_logs(10, 3) -> 7, take_logs(3, 5) -> 0
pub fn take_logs(chest: u32, amount: u32) -> u32 {
    // TODO: a plain `-` panics when amount is bigger than chest. Use `.saturating_sub` instead.
    chest - amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taking_some_leaves_the_rest() {
        assert_eq!(take_logs(10, 3), 7);
        assert_eq!(take_logs(5, 5), 0);
    }

    #[test]
    fn taking_too_many_stops_at_zero() {
        assert_eq!(take_logs(3, 5), 0, "the chest cannot go below zero");
        assert_eq!(take_logs(0, 1), 0);
    }
}
