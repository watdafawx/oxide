pub mod meter;

use meter::allocations;

/// Runs `f` once and returns its result together with how many allocations it made.
pub fn allocations_in<R>(f: impl FnOnce() -> R) -> (R, usize) {
    // TODO: read the meter, run `f`, read the meter again, and return f's result with the
    // difference between the two readings
    let _ = allocations;
    let _ = f;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_arithmetic_allocates_nothing() {
        let (items, allocs) = allocations_in(|| 64 * 27);
        assert_eq!(items, 1728);
        assert_eq!(allocs, 0, "multiplying numbers needs no heap");
    }

    #[test]
    fn a_reserved_chunk_list_is_one_allocation() {
        let (chunks, allocs) = allocations_in(|| Vec::<u32>::with_capacity(100));
        assert_eq!(chunks.capacity(), 100);
        assert_eq!(allocs, 1, "with_capacity asks for its room once");
    }

    #[test]
    fn a_growing_chunk_list_allocates_again_and_again() {
        let (chunks, allocs) = allocations_in(|| {
            let mut chunks = Vec::new();
            for id in 0..100u32 {
                chunks.push(id);
            }
            chunks
        });
        assert_eq!(chunks.len(), 100);
        println!("pushing 100 chunk ids one by one: {allocs} allocations");
        assert!(allocs >= 5, "pushing 100 chunk ids into Vec::new() made only {allocs} allocations");
    }
}
