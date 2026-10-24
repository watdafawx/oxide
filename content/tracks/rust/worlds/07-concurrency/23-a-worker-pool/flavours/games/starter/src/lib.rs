use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

/// Stand-in for hashing a jar of `size` bytes. Takes 20 ms: given, do not change.
pub fn slow_hash(size: u64) -> u64 {
    thread::sleep(Duration::from_millis(20));
    size * 31
}

/// Hashes every jar size with a pool of `workers` threads that share one job queue.
/// Returns the hashes sorted.
pub fn hash_with_pool(sizes: Vec<u64>, workers: usize) -> Vec<u64> {
    let (job_tx, job_rx) = mpsc::channel::<u64>();
    let jobs = Arc::new(Mutex::new(job_rx));
    let (res_tx, res_rx) = mpsc::channel::<u64>();

    // TODO: start `workers` threads, each with an Arc::clone of `jobs` and a clone of `res_tx`.
    // Each worker loops: let job = lock + recv (the guard ends at the ;); on Ok(size) send
    // slow_hash(size) back, on Err break.
    // Then send every size, drop job_tx and res_tx, collect res_rx and sort.
    let _ = (sizes, workers, job_tx, jobs, res_tx, res_rx);
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Instant;

    /// Runs `f` on a helper thread; fails instead of hanging if it takes too long.
    fn within<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(f());
        });
        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(v) => v,
            Err(RecvTimeoutError::Timeout) => {
                panic!("took over 10 s: did a channel never close (a Sender still alive)?")
            }
            Err(RecvTimeoutError::Disconnected) => panic!("the function panicked (see above)"),
        }
    }

    #[test]
    fn pool_hashes_every_jar() {
        let sizes: Vec<u64> = (1..=40).rev().collect();
        let start = Instant::now();
        let got = within(move || hash_with_pool(sizes, 4));
        let want: Vec<u64> = (1..=40).map(|n| n * 31).collect();
        assert_eq!(got, want);
        assert!(
            start.elapsed() < Duration::from_millis(700),
            "40 jars of 20 ms on 4 workers should take about 200 ms, took {:?}: is the lock held while a worker computes?",
            start.elapsed()
        );
    }

    #[test]
    fn pool_finishes_with_odd_worker_counts() {
        assert!(within(|| hash_with_pool(Vec::new(), 4)).is_empty(), "empty mods folder");
        assert_eq!(within(|| hash_with_pool(vec![2, 1, 3], 1)), vec![31, 62, 93], "one worker");
        assert_eq!(within(|| hash_with_pool(vec![2, 1, 3], 16)), vec![31, 62, 93], "more workers than jars");
    }
}
