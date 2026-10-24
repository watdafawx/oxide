//! The macro lives in `count_loads_macro/`. This crate uses it and tests it.
pub use count_loads_macro::count_loads;

#[cfg(test)]
mod tests {
    use super::count_loads;

    #[count_loads]
    fn load_chunk() {}

    #[count_loads]
    fn save_chunk() {}

    #[count_loads]
    fn unload_chunk() {}

    #[test]
    fn each_function_has_its_own_counter() {
        load_chunk();
        load_chunk();
        load_chunk();
        assert_eq!(load_chunk_calls(), 3);
    }

    #[test]
    fn counters_do_not_mix() {
        save_chunk();
        save_chunk();
        unload_chunk();
        assert_eq!(save_chunk_calls(), 2);
        assert_eq!(unload_chunk_calls(), 1);
    }
}
