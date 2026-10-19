use std::sync::RwLock;

/// How many mods are listed (read access only).
pub fn mod_count(mods: &RwLock<Vec<String>>) -> usize {
    // TODO: read lock, then len
    todo!()
}

/// Adds a mod to the list (needs write access).
pub fn add_mod(mods: &RwLock<Vec<String>>, name: &str) {
    // TODO: write lock, then push
    todo!()
}

/// Takes one read guard, then tries for a second one while holding the first.
/// Returns true if the second read lock was granted.
pub fn two_scanners_share(lock: &RwLock<String>) -> bool {
    // TODO: hold a read guard, then try_read()
    todo!()
}

/// Takes a read guard, then tries (without blocking) to get a write lock.
/// Returns true if the write lock was refused.
pub fn installer_is_refused(lock: &RwLock<String>) -> bool {
    // TODO: hold a read guard, then try_write()
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
    fn mod_count_reads_the_list() {
        let mods = Arc::new(RwLock::new(vec!["sodium.jar".to_string(), "iris.jar".to_string()]));
        let m = Arc::clone(&mods);
        assert_eq!(within(move || mod_count(&m)), 2);
    }

    #[test]
    fn add_mod_appends_under_write_lock() {
        let mods = Arc::new(RwLock::new(Vec::new()));
        let m = Arc::clone(&mods);
        within(move || {
            add_mod(&m, "lithium.jar");
            add_mod(&m, "jei.jar");
        });
        assert_eq!(*mods.read().unwrap(), vec!["lithium.jar", "jei.jar"]);
    }

    #[test]
    fn two_scanners_share_the_list() {
        let lock = Arc::new(RwLock::new(String::from("mods")));
        let l = Arc::clone(&lock);
        assert!(within(move || two_scanners_share(&l)), "a second reader should be allowed");
    }

    #[test]
    fn installer_is_refused_while_a_scanner_reads() {
        let lock = Arc::new(RwLock::new(String::from("mods")));
        let l = Arc::clone(&lock);
        assert!(
            within(move || installer_is_refused(&l)),
            "try_write should fail while a read guard lives"
        );
        assert!(lock.try_write().is_ok(), "the guard must be released when the function returns");
    }
}
