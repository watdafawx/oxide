use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Hands out the next job id: adds 1 and returns the value the counter had BEFORE.
pub fn next_job_id(counter: &AtomicUsize) -> usize {
    counter.fetch_add(1, Ordering::Relaxed)
}

/// `workers` threads each add 1 to one shared AtomicUsize `jars_each` times.
/// Returns the final count (no Mutex).
pub fn count_hashed(workers: usize, jars_each: usize) -> usize {
    let hashed = Arc::new(AtomicUsize::new(0));
    let handles: Vec<_> = (0..workers)
        .map(|_| {
            let hashed = Arc::clone(&hashed);
            thread::spawn(move || {
                for _ in 0..jars_each {
                    hashed.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    hashed.load(Ordering::SeqCst)
}

/// Flips `failed` from false to true. Returns true only for the worker that did the flip,
/// so only one worker reports the first failure.
pub fn report_first_failure(failed: &AtomicBool) -> bool {
    failed
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_job_id_returns_the_previous_value() {
        let c = AtomicUsize::new(0);
        assert_eq!(next_job_id(&c), 0);
        assert_eq!(next_job_id(&c), 1);
        assert_eq!(next_job_id(&c), 2);
        assert_eq!(c.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn count_hashed_never_loses_an_update() {
        assert_eq!(count_hashed(4, 5000), 20_000);
        assert_eq!(count_hashed(0, 5), 0);
    }

    #[test]
    fn report_first_failure_has_exactly_one_winner() {
        let failed = AtomicBool::new(false);
        let winners = thread::scope(|s| {
            let handles: Vec<_> = (0..8).map(|_| s.spawn(|| report_first_failure(&failed))).collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .filter(|&won| won)
                .count()
        });
        assert_eq!(winners, 1);
        assert!(failed.load(Ordering::SeqCst));
        assert!(!report_first_failure(&failed), "the failure was already reported");
    }
}
