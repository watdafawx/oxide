use std::time::Duration;
use tokio::time::{sleep, timeout};

/// Given: pretend download that takes `ms` milliseconds, then answers "done".
pub async fn download(ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    "done"
}

/// Some("done") if download(work_ms) finishes within limit_ms, otherwise None.
pub async fn with_deadline(work_ms: u64, limit_ms: u64) -> Option<&'static str> {
    timeout(Duration::from_millis(limit_ms), download(work_ms)).await.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn with_deadline_keeps_a_fast_answer() {
        let fast = timeout(Duration::from_secs(3), with_deadline(10, 500)).await.expect("timed out");
        assert_eq!(fast, Some("done"));
    }

    #[tokio::test]
    async fn with_deadline_gives_up_at_the_limit() {
        let start = Instant::now();
        let slow = timeout(Duration::from_secs(3), with_deadline(2_000, 50)).await.expect("timed out");
        assert_eq!(slow, None);
        assert!(start.elapsed() < Duration::from_millis(1_000), "must give up at the limit, took {:?}", start.elapsed());
    }
}
