use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `threads` workers; each adds 1 to one shared counter `per_thread` times.
/// Returns the final count.
pub fn count_with_workers(threads: u32, per_thread: u32) -> u32 {
    // TODO: one Arc<Mutex<u32>>; clone the Arc for each thread and move the clone in;
    // join every handle, then read the count
    let _ = (threads, per_thread);
    0
}

/// One worker per jar; each pushes "hashed <jar>" into one shared Vec.
/// Returns the lines sorted.
pub fn hashed_jars(jars: Vec<String>) -> Vec<String> {
    // TODO: one Arc<Mutex<Vec<String>>>; one worker per jar pushes its line;
    // join them all, then return a sorted copy of the Vec
    let _ = jars;
    Vec::new()
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
