use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// One more jar hashed. Returns the count from BEFORE this one.
pub fn jar_done(done: &AtomicUsize) -> usize {
    done.fetch_add(1, Ordering::Relaxed)
}

/// `threads` scoped workers each call `jar_done` `per_thread` times on ONE shared
/// AtomicUsize (no Mutex). Returns the final count.
pub fn count_in_workers(threads: usize, per_thread: usize) -> usize {
    let done = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                for _ in 0..per_thread {
                    jar_done(&done);
                }
            });
        }
    });
    done.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jar_done_returns_the_count_from_before() {
        let done = AtomicUsize::new(36);
        assert_eq!(jar_done(&done), 36, "fetch_add hands back the value from before");
        assert_eq!(done.load(Ordering::Relaxed), 37);
    }

    #[test]
    fn count_in_workers_never_loses_an_update() {
        assert_eq!(count_in_workers(8, 10_000), 80_000);
        assert_eq!(count_in_workers(0, 5), 0);
    }
}
