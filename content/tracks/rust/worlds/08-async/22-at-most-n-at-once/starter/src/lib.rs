use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;

/// Given: counts the downloads running right now, the most that ever ran at once, and how many finished.
pub struct Tracker {
    active: AtomicUsize,
    pub peak: AtomicUsize,
    pub finished: AtomicUsize,
}

impl Tracker {
    pub fn new() -> Tracker {
        Tracker { active: AtomicUsize::new(0), peak: AtomicUsize::new(0), finished: AtomicUsize::new(0) }
    }
}

/// Given: pretend download of one episode. Takes 50 ms while the tracker counts it.
pub async fn fake_download(tracker: &Tracker) {
    let now = tracker.active.fetch_add(1, Ordering::SeqCst) + 1;
    tracker.peak.fetch_max(now, Ordering::SeqCst);
    sleep(Duration::from_millis(50)).await;
    tracker.active.fetch_sub(1, Ordering::SeqCst);
    tracker.finished.fetch_add(1, Ordering::SeqCst);
}

/// Downloads `count` episodes, each in its own task, but never more than `limit` at the same time.
pub async fn download_limited(count: usize, limit: usize, tracker: Arc<Tracker>) {
    // TODO: make a semaphore with `limit` permits, shared through an Arc
    // TODO: spawn one task per episode; each takes its own permit, keeps it while it runs
    // fake_download(&tracker), and gives it back at the end
    // TODO: await every handle
    let _ = (count, limit, tracker, Semaphore::new(0));
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn every_episode_is_downloaded() {
        let tracker = Arc::new(Tracker::new());
        timeout(Duration::from_secs(5), download_limited(6, 2, Arc::clone(&tracker)))
            .await
            .expect("timed out");
        assert_eq!(tracker.finished.load(Ordering::SeqCst), 6);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn never_more_than_the_limit() {
        let tracker = Arc::new(Tracker::new());
        timeout(Duration::from_secs(5), download_limited(6, 2, Arc::clone(&tracker)))
            .await
            .expect("timed out");
        let peak = tracker.peak.load(Ordering::SeqCst);
        assert!(peak <= 2, "{peak} downloads ran at once, the limit is 2");
        assert_eq!(peak, 2, "downloads must still run side by side, not one by one");
        assert_eq!(tracker.finished.load(Ordering::SeqCst), 6);
    }
}
