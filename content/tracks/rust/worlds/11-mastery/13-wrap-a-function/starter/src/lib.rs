//! The macro lives in `counted_macro/`. This crate uses it and tests it.
pub use counted_macro::counted;

#[cfg(test)]
mod tests {
    use super::counted;

    /// Adds two numbers, and counts how often it was asked to.
    #[counted]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn counts_each_call_and_keeps_the_result() {
        assert_eq!(calls(), 0);
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(10, -4), 6);
        assert_eq!(add(0, 0), 0);
        assert_eq!(calls(), 3, "add ran 3 times, so calls() must say 3");
    }
}
