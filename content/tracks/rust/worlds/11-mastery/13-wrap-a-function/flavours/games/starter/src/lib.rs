//! The macro lives in `count_loads_macro/`. This crate uses it and tests it.
pub use count_loads_macro::count_loads;

#[cfg(test)]
mod tests {
    use super::count_loads;

    /// Pretends to read a chunk from disk: returns the chunk's index in its region.
    #[count_loads]
    fn load_chunk(x: u32, z: u32) -> u32 {
        x + z * 32
    }

    #[test]
    fn counts_each_load_and_keeps_the_result() {
        assert_eq!(calls(), 0);
        assert_eq!(load_chunk(0, 0), 0);
        assert_eq!(load_chunk(5, 1), 37);
        assert_eq!(load_chunk(31, 31), 1023);
        assert_eq!(calls(), 3, "load_chunk ran 3 times, so calls() must say 3");
    }
}
