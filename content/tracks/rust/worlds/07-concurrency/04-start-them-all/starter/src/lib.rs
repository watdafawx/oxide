use std::thread;
use std::time::Duration;

/// Stand-in for hashing a file of `size` bytes. Takes 150 ms: given, do not change.
pub fn slow_checksum(size: u64) -> u64 {
    thread::sleep(Duration::from_millis(150));
    size * 31
}

/// Checks every size on its own thread. Results come back in the order of `sizes`.
pub fn check_all(sizes: Vec<u64>) -> Vec<u64> {
    // TODO: first spawn one thread per size and keep every handle in a Vec,
    // then join the handles in order and collect the answers
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn check_all_keeps_the_input_order() {
        assert_eq!(check_all(vec![3, 1, 4]), vec![93, 31, 124]);
        assert!(check_all(Vec::new()).is_empty());
    }

    #[test]
    fn check_all_checks_the_files_at_the_same_time() {
        let start = Instant::now();
        let results = check_all(vec![1, 2, 3, 4, 5, 6]);
        let took = start.elapsed();
        assert_eq!(results.len(), 6);
        assert!(
            took < Duration::from_millis(600),
            "six checks took {took:?}; one after another would be 900 ms. Start every thread before you join any."
        );
    }
}
