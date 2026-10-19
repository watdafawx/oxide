use std::sync::Mutex;

/// Adds 1 to the counter.
pub fn bump(counter: &Mutex<u32>) {
    *counter.lock().unwrap() += 1;
}

/// Appends a line to the log.
pub fn push_line(log: &Mutex<Vec<String>>, line: &str) {
    log.lock().unwrap().push(line.to_string());
}

/// Removes and returns all lines, leaving the log empty.
pub fn take_all(log: &Mutex<Vec<String>>) -> Vec<String> {
    std::mem::take(&mut *log.lock().unwrap())
}

/// Returns the value even when a panic poisoned the lock.
pub fn read_even_if_poisoned(m: &Mutex<i32>) -> i32 {
    *m.lock().unwrap_or_else(|e| e.into_inner())
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
    fn bump_adds_one_under_the_lock() {
        let counter = Arc::new(Mutex::new(0u32));
        let c = Arc::clone(&counter);
        within(move || {
            bump(&c);
            bump(&c);
            bump(&c);
        });
        assert_eq!(*counter.lock().unwrap(), 3);
    }

    #[test]
    fn push_and_take_all_empties_the_log() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = Arc::clone(&log);
        let taken = within(move || {
            push_line(&l, "one");
            push_line(&l, "two");
            take_all(&l)
        });
        assert_eq!(taken, vec!["one", "two"]);
        assert!(log.lock().unwrap().is_empty(), "the log should be empty after take_all");
    }

    #[test]
    fn read_even_if_poisoned_recovers_the_value() {
        let m = Arc::new(Mutex::new(5));
        let poisoner = Arc::clone(&m);
        let result = thread::spawn(move || {
            let _guard = poisoner.lock().unwrap();
            panic!("this panic is on purpose: it poisons the mutex");
        })
        .join();
        assert!(result.is_err());
        assert!(m.is_poisoned());
        let m2 = Arc::clone(&m);
        assert_eq!(within(move || read_even_if_poisoned(&m2)), 5);
    }
}
