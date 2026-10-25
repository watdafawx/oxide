use std::sync::Mutex;

/// How many jars are hashed, even if a worker panicked while holding the lock.
pub fn jars_hashed(done: &Mutex<u32>) -> u32 {
    *done.lock().unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn jars_hashed_reads_a_poisoned_counter() {
        let done = Mutex::new(41);
        assert_eq!(jars_hashed(&done), 41);

        thread::scope(|s| {
            let crashed = s.spawn(|| {
                let mut guard = done.lock().unwrap();
                *guard += 1;
                panic!("a broken jar, while holding the lock");
            });
            assert!(crashed.join().is_err());
        });
        assert!(done.is_poisoned(), "the test should have poisoned the lock");
        assert_eq!(jars_hashed(&done), 42, "read the count even though the lock is poisoned");
    }
}
