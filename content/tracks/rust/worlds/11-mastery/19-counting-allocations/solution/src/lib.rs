pub mod meter;

use meter::allocations;

/// Runs `f` once and returns its result together with how many allocations it made.
pub fn allocations_in<R>(f: impl FnOnce() -> R) -> (R, usize) {
    let before = allocations();
    let result = f();
    let after = allocations();
    (result, after - before)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_arithmetic_allocates_nothing() {
        let (sum, allocs) = allocations_in(|| 40 + 2);
        assert_eq!(sum, 42);
        assert_eq!(allocs, 0, "adding numbers needs no heap");
    }

    #[test]
    fn a_reserved_vec_is_one_allocation() {
        let (list, allocs) = allocations_in(|| Vec::<u32>::with_capacity(100));
        assert_eq!(list.capacity(), 100);
        assert_eq!(allocs, 1, "with_capacity asks for its room once");
    }

    #[test]
    fn a_growing_vec_allocates_again_and_again() {
        let (list, allocs) = allocations_in(|| {
            let mut list = Vec::new();
            for n in 0..100u32 {
                list.push(n);
            }
            list
        });
        assert_eq!(list.len(), 100);
        println!("pushing 100 items one by one: {allocs} allocations");
        assert!(allocs >= 5, "pushing 100 items into Vec::new() made only {allocs} allocations");
    }
}
