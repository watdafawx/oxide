use std::sync::RwLock;

/// Number of entries (read access only).
pub fn entry_count(cfg: &RwLock<Vec<String>>) -> usize {
    // TODO: cfg.read()
    todo!()
}

/// Appends an entry (needs write access).
pub fn add_entry(cfg: &RwLock<Vec<String>>, entry: &str) {
    // TODO: cfg.write()
    todo!()
}

/// Takes one read guard, then tries for a second one while holding the first.
/// Returns true if the second read lock was granted.
pub fn readers_share(lock: &RwLock<String>) -> bool {
    // TODO: let _first = lock.read()...; then lock.try_read().is_ok()
    todo!()
}

/// Takes a read guard, then tries (without blocking) to get a write lock.
/// Returns true if the write lock was refused.
pub fn writer_is_refused(lock: &RwLock<String>) -> bool {
    // TODO: hold a read guard; use try_write(), never write() (that would deadlock)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{self, RecvTimeoutError};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    /// Runs `f` on a helper thread and fails (instead of hanging) if it takes too long.
    fn within<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(f());
        });
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(v) => v,
            Err(RecvTimeoutError::Timeout) => {
                panic!("took over 5 s: asking for write() while holding a read guard deadlocks")
            }
            Err(RecvTimeoutError::Disconnected) => panic!("the function panicked (see above)"),
        }
    }

    #[test]
    fn entry_count_reads_the_list() {
        let cfg = Arc::new(RwLock::new(vec!["a".to_string(), "b".to_string()]));
        let c = Arc::clone(&cfg);
        assert_eq!(within(move || entry_count(&c)), 2);
    }

    #[test]
    fn add_entry_appends_under_write_lock() {
        let cfg = Arc::new(RwLock::new(Vec::new()));
        let c = Arc::clone(&cfg);
        within(move || {
            add_entry(&c, "x");
            add_entry(&c, "y");
        });
        assert_eq!(*cfg.read().unwrap(), vec!["x", "y"]);
    }

    #[test]
    fn readers_share_the_lock() {
        let lock = Arc::new(RwLock::new(String::from("cfg")));
        let l = Arc::clone(&lock);
        assert!(within(move || readers_share(&l)), "a second reader should be allowed");
    }

    #[test]
    fn writer_is_refused_while_a_reader_holds_the_lock() {
        let lock = Arc::new(RwLock::new(String::from("cfg")));
        let l = Arc::clone(&lock);
        assert!(within(move || writer_is_refused(&l)), "try_write should fail while a read guard lives");
        assert!(lock.try_write().is_ok(), "the guard must be released when the function returns");
    }
}
