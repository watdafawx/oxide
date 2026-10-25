use std::sync::RwLock;

/// How many checksums are in the table (read access only).
pub fn known_count(known: &RwLock<Vec<String>>) -> usize {
    // TODO: take a read lock and return the length
    let _ = known;
    0
}

/// Adds a checksum to the table (needs write access).
pub fn remember(known: &RwLock<Vec<String>>, sum: &str) {
    // TODO: take a write lock and push sum as a String
    let _ = (known, sum);
}

/// Holds a read guard, then asks for a write lock without waiting.
/// Returns true if the write lock was refused.
pub fn writer_is_refused(lock: &RwLock<Vec<String>>) -> bool {
    // TODO: keep a read guard alive in a variable, then try_write() (never write(): that would deadlock)
    let _ = lock;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> RwLock<Vec<String>> {
        RwLock::new(vec![String::from("a1f3"), String::from("09bc")])
    }

    #[test]
    fn known_count_reads_the_table() {
        assert_eq!(known_count(&table()), 2);
    }

    #[test]
    fn remember_appends_under_a_write_lock() {
        let known = table();
        remember(&known, "77de");
        assert_eq!(*known.read().unwrap(), vec!["a1f3", "09bc", "77de"]);
    }

    #[test]
    fn writer_is_refused_while_someone_reads() {
        let known = table();
        assert!(writer_is_refused(&known), "a writer must not get in while a reader holds the lock");
        assert!(known.try_write().is_ok(), "the read guard should be gone after the function returns");
    }
}
