use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::sleep;

/// Pretend download: sleeps, then returns "done". Given.
pub async fn download(ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    "done"
}

/// Some("done") if download(work_ms) finishes within limit_ms, else None.
pub async fn download_with_timeout(work_ms: u64, limit_ms: u64) -> Option<&'static str> {
    tokio::time::timeout(Duration::from_millis(limit_ms), download(work_ms))
        .await
        .ok()
}

/// Sleeps a_ms and b_ms at the same time. Returns "mirror-a" or "mirror-b", whichever ends first.
pub async fn fastest_mirror(a_ms: u64, b_ms: u64) -> &'static str {
    tokio::select! {
        _ = sleep(Duration::from_millis(a_ms)) => "mirror-a",
        _ = sleep(Duration::from_millis(b_ms)) => "mirror-b",
    }
}

/// Runs download(work_ms). If `cancel` fires first, stop and return "cancelled".
pub async fn download_or_cancel(work_ms: u64, mut cancel: oneshot::Receiver<()>) -> &'static str {
    tokio::select! {
        _ = &mut cancel => "cancelled",
        result = download(work_ms) => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn download_with_timeout_gives_up_in_time() {
        let fast = timeout(Duration::from_secs(3), download_with_timeout(10, 500))
            .await
            .expect("timed out");
        assert_eq!(fast, Some("done"));
        let start = Instant::now();
        let slow = timeout(Duration::from_secs(3), download_with_timeout(2000, 50))
            .await
            .expect("timed out");
        assert_eq!(slow, None);
        assert!(start.elapsed() < Duration::from_millis(1000), "must give up at the limit");
    }

    #[tokio::test]
    async fn fastest_mirror_returns_the_quicker_one() {
        let a = timeout(Duration::from_secs(3), fastest_mirror(10, 800)).await.expect("timed out");
        assert_eq!(a, "mirror-a");
        let b = timeout(Duration::from_secs(3), fastest_mirror(800, 10)).await.expect("timed out");
        assert_eq!(b, "mirror-b");
    }

    #[tokio::test]
    async fn download_or_cancel_obeys_cancel() {
        let (tx, rx) = oneshot::channel();
        let task = tokio::spawn(download_or_cancel(5000, rx));
        sleep(Duration::from_millis(30)).await;
        tx.send(()).unwrap();
        let out = timeout(Duration::from_secs(2), task).await.expect("did not stop").unwrap();
        assert_eq!(out, "cancelled");
    }

    #[tokio::test]
    async fn download_or_cancel_finishes_normally() {
        let (_tx, rx) = oneshot::channel::<()>(); // sender kept alive, never fires
        let out = timeout(Duration::from_secs(2), download_or_cancel(20, rx))
            .await
            .expect("timed out");
        assert_eq!(out, "done");
    }
}
