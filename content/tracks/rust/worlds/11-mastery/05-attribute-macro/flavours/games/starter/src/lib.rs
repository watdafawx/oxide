//! The macro lives in `count_calls_macro/`. This crate re-exports it and tests it.
pub use timed_macro::count_calls;

#[cfg(test)]
mod tests {
    use super::count_calls;

    #[count_calls]
    fn load_chunk() {}

    #[count_calls]
    fn save_chunk() {}

    #[count_calls]
    fn unload_chunk() {}

    #[count_calls]
    fn blocks_in(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn counts_each_call() {
        assert_eq!(load_chunk_calls(), 0);
        load_chunk();
        load_chunk();
        load_chunk();
        assert_eq!(load_chunk_calls(), 3);
    }

    #[test]
    fn counts_are_per_function() {
        save_chunk();
        save_chunk();
        unload_chunk();
        assert_eq!(save_chunk_calls(), 2);
        assert_eq!(unload_chunk_calls(), 1);
    }

    #[test]
    fn keeps_behaviour_and_counts() {
        assert_eq!(blocks_in(2, 3), 5);
        assert_eq!(blocks_in_calls(), 1);
    }
}
