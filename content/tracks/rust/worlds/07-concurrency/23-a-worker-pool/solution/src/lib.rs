use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

/// Stand-in for hashing a file of `size` bytes. Takes 20 ms: given, do not change.
pub fn slow_checksum(size: u64) -> u64 {
    thread::sleep(Duration::from_millis(20));
    size * 31
}

/// Checks every size with a pool of `workers` threads that share one job queue.
/// Returns the checksums sorted.
pub fn check_with_pool(sizes: Vec<u64>, workers: usize) -> Vec<u64> {
    let (job_tx, job_rx) = mpsc::channel::<u64>();
    let jobs = Arc::new(Mutex::new(job_rx));
    let (res_tx, res_rx) = mpsc::channel::<u64>();

    for _ in 0..workers {
        let jobs = Arc::clone(&jobs);
        let results = res_tx.clone();
        thread::spawn(move || {
            loop {
                let job = jobs.lock().unwrap().recv();
                match job {
                    Ok(size) => results.send(slow_checksum(size)).unwrap(),
                    Err(_) => break,
                }
            }
        });
    }
    for size in sizes {
        job_tx.send(size).unwrap();
    }
    drop(job_tx);
    drop(res_tx);

    let mut out: Vec<u64> = res_rx.iter().collect();
    out.sort();
    out
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
    fn pool_checks_every_size() {
        let sizes: Vec<u64> = (1..=40).rev().collect();
        let start = Instant::now();
        let got = within(move || check_with_pool(sizes, 4));
        let want: Vec<u64> = (1..=40).map(|n| n * 31).collect();
        assert_eq!(got, want);
        assert!(
            start.elapsed() < Duration::from_millis(700),
            "40 jobs of 20 ms on 4 workers should take about 200 ms, took {:?}: is the lock held while a worker computes?",
            start.elapsed()
        );
    }

    #[test]
    fn pool_finishes_with_odd_worker_counts() {
        assert!(within(|| check_with_pool(Vec::new(), 4)).is_empty(), "no jobs");
        assert_eq!(within(|| check_with_pool(vec![2, 1, 3], 1)), vec![31, 62, 93], "one worker");
        assert_eq!(within(|| check_with_pool(vec![2, 1, 3], 16)), vec![31, 62, 93], "more workers than jobs");
    }
}
