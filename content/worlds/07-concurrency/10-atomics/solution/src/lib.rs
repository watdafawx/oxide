use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Adds 1 and returns the value the counter had BEFORE the increment.
pub fn increment(counter: &AtomicUsize) -> usize {
    counter.fetch_add(1, Ordering::Relaxed)
}

/// `threads` threads each add 1 to one shared AtomicUsize `per_thread` times.
/// Returns the final value (no Mutex).
pub fn count_in_threads(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    increment(&counter);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    counter.load(Ordering::SeqCst)
}

/// Flips `flag` from false to true. Returns true only for the caller that did the flip.
pub fn claim_once(flag: &AtomicBool) -> bool {
    flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_returns_the_previous_value() {
        let c = AtomicUsize::new(0);
        assert_eq!(increment(&c), 0);
        assert_eq!(increment(&c), 1);
        assert_eq!(increment(&c), 2);
        assert_eq!(c.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn count_in_threads_never_loses_an_update() {
        assert_eq!(count_in_threads(4, 5000), 20_000);
        assert_eq!(count_in_threads(0, 5), 0);
    }

    #[test]
    fn claim_once_has_exactly_one_winner() {
        let flag = AtomicBool::new(false);
        let winners = thread::scope(|s| {
            let handles: Vec<_> = (0..8).map(|_| s.spawn(|| claim_once(&flag))).collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .filter(|&won| won)
                .count()
        });
        assert_eq!(winners, 1);
        assert!(flag.load(Ordering::SeqCst));
        assert!(!claim_once(&flag), "the flag is already taken");
    }
}
