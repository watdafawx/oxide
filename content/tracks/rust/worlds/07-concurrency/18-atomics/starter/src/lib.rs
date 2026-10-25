use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// One more file finished. Returns the count from BEFORE this one.
pub fn file_done(done: &AtomicUsize) -> usize {
    // TODO: add 1 with fetch_add (Ordering::Relaxed) and return what it hands back
    let _ = done;
    0
}

/// `threads` scoped threads each call `file_done` `per_thread` times on ONE shared
/// AtomicUsize (no Mutex). Returns the final count.
pub fn count_in_threads(threads: usize, per_thread: usize) -> usize {
    // TODO: make an AtomicUsize at 0; inside thread::scope start `threads` threads that
    // each call file_done(&done) per_thread times; then load the value
    let _ = (threads, per_thread);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_done_returns_the_count_from_before() {
        let done = AtomicUsize::new(36);
        assert_eq!(file_done(&done), 36, "fetch_add hands back the value from before");
        assert_eq!(done.load(Ordering::Relaxed), 37);
    }

    #[test]
    fn count_in_threads_never_loses_an_update() {
        assert_eq!(count_in_threads(8, 10_000), 80_000);
        assert_eq!(count_in_threads(0, 5), 0);
    }
}
