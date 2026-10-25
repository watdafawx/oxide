use std::sync::Mutex;

/// One more jar hashed: adds 1 to the shared counter.
pub fn bump(done: &Mutex<u32>) {
    *done.lock().unwrap() += 1;
}

/// Appends a line to the shared log.
pub fn log_line(log: &Mutex<Vec<String>>, line: &str) {
    log.lock().unwrap().push(line.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn bump_never_loses_a_jar() {
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
        assert_eq!(*shared.lock().unwrap(), 1000, "four workers, 250 jars each");
    }

    #[test]
    fn log_line_appends_to_the_launcher_log() {
        let log = Mutex::new(vec![String::from("start")]);
        log_line(&log, "hashed sodium.jar");
        log_line(&log, "hashed iris.jar");
        assert_eq!(*log.lock().unwrap(), vec!["start", "hashed sodium.jar", "hashed iris.jar"]);
    }
}
