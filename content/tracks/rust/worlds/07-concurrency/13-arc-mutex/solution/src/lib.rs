use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `threads` threads; each adds 1 to one shared counter `per_thread` times.
/// Returns the final count.
pub fn count_with_threads(threads: u32, per_thread: u32) -> u32 {
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

/// One thread per name; each pushes "checked <name>" into one shared Vec.
/// Returns the lines sorted.
pub fn checked_names(names: Vec<String>) -> Vec<String> {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for name in names {
        let mine = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            mine.lock().unwrap().push(format!("checked {name}"));
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
    fn count_with_threads_adds_up() {
        assert_eq!(count_with_threads(4, 250), 1000);
        assert_eq!(count_with_threads(1, 3), 3);
    }

    #[test]
    fn checked_names_gathers_every_thread() {
        let names = vec![String::from("photo.jpg"), String::from("a.txt"), String::from("notes.md")];
        assert_eq!(
            checked_names(names),
            vec!["checked a.txt", "checked notes.md", "checked photo.jpg"]
        );
        assert!(checked_names(Vec::new()).is_empty());
    }
}
