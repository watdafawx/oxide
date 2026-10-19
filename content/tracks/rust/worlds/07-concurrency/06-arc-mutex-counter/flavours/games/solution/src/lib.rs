use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `workers` threads; each adds 1 to a shared files-done counter
/// `jars_each` times. Returns the final count.
pub fn hash_counter(workers: usize, jars_each: usize) -> usize {
    let done = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..workers)
        .map(|_| {
            let done = Arc::clone(&done);
            thread::spawn(move || {
                for _ in 0..jars_each {
                    *done.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    Arc::try_unwrap(done).unwrap().into_inner().unwrap()
}

/// Worker `i` (0..n) pushes the stand-in hash `i * i` into one shared Vec.
/// Returns the Vec sorted.
pub fn collect_hashes(n: u32) -> Vec<u32> {
    let hashes = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (0..n)
        .map(|i| {
            let hashes = Arc::clone(&hashes);
            thread::spawn(move || hashes.lock().unwrap().push(i * i))
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let mut out = Arc::try_unwrap(hashes).unwrap().into_inner().unwrap();
    out.sort();
    out
}

/// One worker per jar; each increments that jar's mod loader ("fabric",
/// "neoforge", ...) in one shared map.
pub fn loader_counts(loaders: &[&str]) -> BTreeMap<String, usize> {
    let counts = Arc::new(Mutex::new(BTreeMap::new()));
    let handles: Vec<_> = loaders
        .iter()
        .map(|loader| {
            let counts = Arc::clone(&counts);
            let loader = loader.to_string();
            thread::spawn(move || {
                *counts.lock().unwrap().entry(loader).or_default() += 1;
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    Arc::try_unwrap(counts).unwrap().into_inner().unwrap()
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
