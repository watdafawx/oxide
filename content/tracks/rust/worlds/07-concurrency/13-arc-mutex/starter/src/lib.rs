use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `threads` threads; each adds 1 to one shared counter `per_thread` times.
/// Returns the final count.
pub fn count_with_threads(threads: u32, per_thread: u32) -> u32 {
    // TODO: one Arc<Mutex<u32>>; clone the Arc for each thread and move the clone in;
    // join every handle, then read the count
    let _ = (threads, per_thread);
    0
}

/// One thread per name; each pushes "checked <name>" into one shared Vec.
/// Returns the lines sorted.
pub fn checked_names(names: Vec<String>) -> Vec<String> {
    // TODO: one Arc<Mutex<Vec<String>>>; one thread per name pushes its line;
    // join them all, then return a sorted copy of the Vec
    let _ = names;
    Vec::new()
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
