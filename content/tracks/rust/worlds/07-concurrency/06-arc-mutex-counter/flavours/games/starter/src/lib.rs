use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `workers` threads; each adds 1 to a shared files-done counter
/// `jars_each` times. Returns the final count.
pub fn hash_counter(workers: usize, jars_each: usize) -> usize {
    // TODO: Arc<Mutex<usize>>, one Arc::clone per worker, join all, then read the value
    todo!()
}

/// Worker `i` (0..n) pushes the stand-in hash `i * i` into one shared Vec.
/// Returns the Vec sorted.
pub fn collect_hashes(n: u32) -> Vec<u32> {
    // TODO: one shared Vec, one worker per i, sort at the end
    todo!()
}

/// One worker per jar; each increments that jar's mod loader ("fabric",
/// "neoforge", ...) in one shared map.
pub fn loader_counts(loaders: &[&str]) -> BTreeMap<String, usize> {
    // TODO: shared BTreeMap<String, usize>; each worker owns its loader String
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{self, RecvTimeoutError};
    use std::time::Duration;

    /// Runs `f` on a helper thread and fails (instead of hanging) if it takes too long.
    fn within<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(f());
        });
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(v) => v,
            Err(RecvTimeoutError::Timeout) => {
                panic!("took over 10 s: is a guard held while you join, or a lock taken twice?")
            }
            Err(RecvTimeoutError::Disconnected) => panic!("the function panicked (see above)"),
        }
    }

    #[test]
    fn hash_counter_never_loses_a_jar() {
        assert_eq!(within(|| hash_counter(4, 1000)), 4000);
        assert_eq!(within(|| hash_counter(0, 10)), 0);
    }

    #[test]
    fn collect_hashes_gathers_every_worker() {
        assert_eq!(within(|| collect_hashes(5)), vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn loader_counts_tally_in_a_shared_map() {
        let counts = within(|| loader_counts(&["fabric", "neoforge", "fabric", "forge", "fabric", "neoforge"]));
        let expected: BTreeMap<String, usize> = [("fabric", 3), ("forge", 1), ("neoforge", 2)]
            .into_iter()
            .map(|(w, n)| (w.to_string(), n))
            .collect();
        assert_eq!(counts, expected);
    }
}
