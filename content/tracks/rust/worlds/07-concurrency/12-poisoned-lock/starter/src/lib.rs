use std::sync::Mutex;

/// How many files are done, even if a worker panicked while holding the lock.
pub fn files_done(done: &Mutex<u32>) -> u32 {
    // TODO: lock() gives Err(PoisonError) after a panic; use unwrap_or_else
    // with a closure that takes the guard out of the error, then return the number
    *done.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn files_done_reads_a_poisoned_counter() {
        let done = Mutex::new(41);
        assert_eq!(files_done(&done), 41);

        thread::scope(|s| {
            let crashed = s.spawn(|| {
                let mut guard = done.lock().unwrap();
                *guard += 1;
                panic!("a broken file, while holding the lock");
            });
            assert!(crashed.join().is_err());
        });
        assert!(done.is_poisoned(), "the test should have poisoned the lock");
        assert_eq!(files_done(&done), 42, "read the count even though the lock is poisoned");
    }
}
