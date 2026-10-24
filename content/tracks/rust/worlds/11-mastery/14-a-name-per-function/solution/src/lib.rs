//! The macro lives in `counted_macro/`. This crate uses it and tests it.
pub use counted_macro::counted;

#[cfg(test)]
mod tests {
    use super::counted;

    #[counted]
    fn ping() {}

    #[counted]
    fn left() {}

    #[counted]
    fn right() {}

    #[test]
    fn each_function_has_its_own_counter() {
        ping();
        ping();
        ping();
        assert_eq!(ping_calls(), 3);
    }

    #[test]
    fn counters_do_not_mix() {
        left();
        left();
        right();
        assert_eq!(left_calls(), 2);
        assert_eq!(right_calls(), 1);
    }
}
