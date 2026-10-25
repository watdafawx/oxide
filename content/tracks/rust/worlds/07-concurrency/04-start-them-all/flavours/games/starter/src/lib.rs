use std::thread;
use std::time::Duration;

/// Stand-in for hashing a jar of `size` bytes. Takes 150 ms: given, do not change.
pub fn slow_hash(size: u64) -> u64 {
    thread::sleep(Duration::from_millis(150));
    size * 31
}

/// Hashes every jar size on its own worker. Hashes come back in folder order.
pub fn hash_all(sizes: Vec<u64>) -> Vec<u64> {
    // TODO: first spawn one worker per size and keep every handle in a Vec,
    // then join the handles in order and collect the hashes
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn hash_all_keeps_the_folder_order() {
        assert_eq!(hash_all(vec![3, 1, 4]), vec![93, 31, 124]);
        assert!(hash_all(Vec::new()).is_empty());
    }

    #[test]
    fn hash_all_hashes_the_jars_at_the_same_time() {
        let start = Instant::now();
        let hashes = hash_all(vec![1, 2, 3, 4, 5, 6]);
        let took = start.elapsed();
        assert_eq!(hashes.len(), 6);
        assert!(
            took < Duration::from_millis(600),
            "six jars took {took:?}; one after another would be 900 ms. Start every worker before you join any."
        );
    }
}
