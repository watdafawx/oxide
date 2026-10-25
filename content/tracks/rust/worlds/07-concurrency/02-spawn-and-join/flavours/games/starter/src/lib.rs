use std::thread::{self, JoinHandle};

/// Stand-in for reading a jar and hashing it: given, do not change.
pub fn quick_hash(jar_name: &str) -> u64 {
    jar_name.bytes().map(u64::from).sum::<u64>() * 31
}

/// Starts hashing "sodium.jar" on a new thread. Does not wait: returns the handle.
pub fn start_hash() -> JoinHandle<u64> {
    // TODO: spawn a thread whose closure computes quick_hash("sodium.jar"); return the handle
    todo!()
}

/// Starts the hash, waits for it to finish, and returns the hash.
pub fn hash_and_wait() -> u64 {
    // TODO: call start_hash, then wait for the thread and take its answer
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_hash_hashes_sodium_on_a_thread() {
        let handle = start_hash();
        assert_eq!(handle.join().unwrap(), quick_hash("sodium.jar"));
    }

    #[test]
    fn hash_and_wait_returns_the_hash() {
        assert_eq!(hash_and_wait(), quick_hash("sodium.jar"));
    }
}
