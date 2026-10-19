use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Starts `threads` threads; each adds 1 to a shared counter `per_thread` times.
/// Returns the final count.
pub fn count_with_threads(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    *counter.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    Arc::try_unwrap(counter).unwrap().into_inner().unwrap()
}

/// Thread `i` (0..n) pushes `i * i` into one shared Vec. Returns the Vec sorted.
pub fn collect_squares(n: u32) -> Vec<u32> {
    let squares = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (0..n)
        .map(|i| {
            let squares = Arc::clone(&squares);
            thread::spawn(move || squares.lock().unwrap().push(i * i))
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let mut out = Arc::try_unwrap(squares).unwrap().into_inner().unwrap();
    out.sort();
    out
}

/// One thread per word; each increments that word's count in one shared map.
pub fn word_counts(words: &[&str]) -> BTreeMap<String, usize> {
    let counts = Arc::new(Mutex::new(BTreeMap::new()));
    let handles: Vec<_> = words
        .iter()
        .map(|word| {
            let counts = Arc::clone(&counts);
            let word = word.to_string();
            thread::spawn(move || {
                *counts.lock().unwrap().entry(word).or_default() += 1;
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
    fn count_with_threads_adds_up() {
        assert_eq!(within(|| count_with_threads(4, 1000)), 4000);
        assert_eq!(within(|| count_with_threads(0, 10)), 0);
    }

    #[test]
    fn collect_squares_gathers_every_thread() {
        assert_eq!(within(|| collect_squares(5)), vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn word_counts_tallies_in_a_shared_map() {
        let counts = within(|| word_counts(&["a", "b", "a", "c", "a", "b"]));
        let expected: BTreeMap<String, usize> = [("a", 3), ("b", 2), ("c", 1)]
            .into_iter()
            .map(|(w, n)| (w.to_string(), n))
            .collect();
        assert_eq!(counts, expected);
    }
}
