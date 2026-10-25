use std::sync::Mutex;

/// One more file finished: adds 1 to the shared counter.
pub fn bump(done: &Mutex<u32>) {
    // TODO: lock the counter and add 1 through the guard
    let _ = done;
}

/// Appends a line to the shared log.
pub fn log_line(log: &Mutex<Vec<String>>, line: &str) {
    // TODO: lock the log and push the line as a String
    let _ = (log, line);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn bump_never_loses_a_file() {
        let done = Mutex::new(0);
        bump(&done);
        assert_eq!(*done.lock().unwrap(), 1);

        let shared = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let mine = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                for _ in 0..250 {
                    bump(&mine);
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(*shared.lock().unwrap(), 1000, "four threads, 250 files each");
    }

    #[test]
    fn log_line_appends_under_the_lock() {
        let log = Mutex::new(vec![String::from("start")]);
        log_line(&log, "hashed a.txt");
        log_line(&log, "hashed b.txt");
        assert_eq!(*log.lock().unwrap(), vec!["start", "hashed a.txt", "hashed b.txt"]);
    }
}
