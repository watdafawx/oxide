use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `threads` workers; each adds 1 to one shared counter `per_thread` times.
/// Returns the final count.
pub fn count_with_workers(threads: u32, per_thread: u32) -> u32 {
    let shared = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..threads {
        let mine = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                *mine.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    *shared.lock().unwrap()
}

/// One worker per jar; each pushes "hashed <jar>" into one shared Vec.
/// Returns the lines sorted.
pub fn hashed_jars(jars: Vec<String>) -> Vec<String> {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for jar in jars {
        let mine = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            mine.lock().unwrap().push(format!("hashed {jar}"));
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut out = shared.lock().unwrap().clone();
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_with_workers_adds_up() {
        assert_eq!(count_with_workers(4, 250), 1000);
        assert_eq!(count_with_workers(1, 3), 3);
    }

    #[test]
    fn hashed_jars_gathers_every_worker() {
        let jars = vec![String::from("sodium.jar"), String::from("create.jar"), String::from("jei.jar")];
        assert_eq!(
            hashed_jars(jars),
            vec!["hashed create.jar", "hashed jei.jar", "hashed sodium.jar"]
        );
        assert!(hashed_jars(Vec::new()).is_empty());
    }
}
