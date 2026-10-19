use std::sync::Mutex;

/// A worker finished one jar: add 1 to the files-done counter.
pub fn mark_hashed(done: &Mutex<u32>) {
    *done.lock().unwrap() += 1;
}

/// Appends a status line ("sodium.jar ok") to the shared log.
pub fn log_line(log: &Mutex<Vec<String>>, line: &str) {
    log.lock().unwrap().push(line.to_string());
}

/// Removes and returns all lines, leaving the log empty.
pub fn take_log(log: &Mutex<Vec<String>>) -> Vec<String> {
    std::mem::take(&mut *log.lock().unwrap())
}

/// Returns files-done even when a worker panic poisoned the lock.
pub fn hashed_count_even_if_poisoned(done: &Mutex<i32>) -> i32 {
    *done.lock().unwrap_or_else(|e| e.into_inner())
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
                panic!("took over 5 s: is a guard still alive when you lock again?")
            }
            Err(RecvTimeoutError::Disconnected) => panic!("the function panicked (see above)"),
        }
    }

    #[test]
    fn mark_hashed_adds_one_under_the_lock() {
        let done = Arc::new(Mutex::new(0u32));
        let d = Arc::clone(&done);
        within(move || {
            mark_hashed(&d);
            mark_hashed(&d);
            mark_hashed(&d);
        });
        assert_eq!(*done.lock().unwrap(), 3);
    }

    #[test]
    fn log_and_take_log_empties_the_log() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = Arc::clone(&log);
        let taken = within(move || {
            log_line(&l, "sodium.jar ok");
            log_line(&l, "iris.jar ok");
            take_log(&l)
        });
        assert_eq!(taken, vec!["sodium.jar ok", "iris.jar ok"]);
        assert!(log.lock().unwrap().is_empty(), "the log should be empty after take_log");
    }

    #[test]
    fn hashed_count_even_if_poisoned_recovers_the_value() {
        let m = Arc::new(Mutex::new(5));
        let poisoner = Arc::clone(&m);
        let result = thread::spawn(move || {
            let _guard = poisoner.lock().unwrap();
            panic!("this panic is on purpose: a worker dies holding the lock");
        })
        .join();
        assert!(result.is_err());
        assert!(m.is_poisoned());
        let m2 = Arc::clone(&m);
        assert_eq!(within(move || hashed_count_even_if_poisoned(&m2)), 5);
    }
}
