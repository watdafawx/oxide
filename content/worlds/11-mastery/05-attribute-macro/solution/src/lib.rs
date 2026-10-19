//! The macro lives in `timed_macro/`. This crate re-exports it and tests it.
pub use timed_macro::timed;

#[cfg(test)]
mod tests {
    use super::timed;

    #[timed]
    fn ping() {}

    #[timed]
    fn left() {}

    #[timed]
    fn right() {}

    #[timed]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn counts_each_call() {
        assert_eq!(ping_calls(), 0);
        ping();
        ping();
        ping();
        assert_eq!(ping_calls(), 3);
    }

    #[test]
    fn counts_are_per_function() {
        left();
        left();
        right();
        assert_eq!(left_calls(), 2);
        assert_eq!(right_calls(), 1);
    }

    #[test]
    fn keeps_behaviour_and_counts() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add_calls(), 1);
    }
}
