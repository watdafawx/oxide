use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::sleep;

/// Pretend work: sleeps, then returns "done". Given.
pub async fn work(ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    "done"
}

/// Some("done") if work(work_ms) finishes within limit_ms, else None.
pub async fn with_deadline(work_ms: u64, limit_ms: u64) -> Option<&'static str> {
    tokio::time::timeout(Duration::from_millis(limit_ms), work(work_ms))
        .await
        .ok()
}

/// Sleeps a_ms and b_ms at the same time. Returns "a" or "b", whichever ends first.
pub async fn first_of(a_ms: u64, b_ms: u64) -> &'static str {
    tokio::select! {
        _ = sleep(Duration::from_millis(a_ms)) => "a",
        _ = sleep(Duration::from_millis(b_ms)) => "b",
    }
}

/// Runs work(work_ms). If `cancel` fires first, stop and return "cancelled".
pub async fn work_or_cancel(work_ms: u64, mut cancel: oneshot::Receiver<()>) -> &'static str {
    tokio::select! {
        _ = &mut cancel => "cancelled",
        result = work(work_ms) => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn with_deadline_gives_up_in_time() {
        let fast = timeout(Duration::from_secs(3), with_deadline(10, 500)).await.expect("timed out");
        assert_eq!(fast, Some("done"));
        let start = Instant::now();
        let slow = timeout(Duration::from_secs(3), with_deadline(2000, 50)).await.expect("timed out");
        assert_eq!(slow, None);
        assert!(start.elapsed() < Duration::from_millis(1000), "must give up at the limit");
    }

    #[tokio::test]
    async fn first_of_returns_the_faster_one() {
        let a = timeout(Duration::from_secs(3), first_of(10, 800)).await.expect("timed out");
        assert_eq!(a, "a");
        let b = timeout(Duration::from_secs(3), first_of(800, 10)).await.expect("timed out");
        assert_eq!(b, "b");
    }

    #[tokio::test]
    async fn work_or_cancel_obeys_cancel() {
        let (tx, rx) = oneshot::channel();
        let task = tokio::spawn(work_or_cancel(5000, rx));
        sleep(Duration::from_millis(30)).await;
        tx.send(()).unwrap();
        let out = timeout(Duration::from_secs(2), task).await.expect("did not stop").unwrap();
        assert_eq!(out, "cancelled");
    }

    #[tokio::test]
    async fn work_or_cancel_finishes_normally() {
        let (_tx, rx) = oneshot::channel::<()>(); // sender kept alive, never fires
        let out = timeout(Duration::from_secs(2), work_or_cancel(20, rx)).await.expect("timed out");
        assert_eq!(out, "done");
    }
}
