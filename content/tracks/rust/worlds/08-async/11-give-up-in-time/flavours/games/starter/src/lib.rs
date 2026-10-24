use std::time::Duration;
use tokio::time::{sleep, timeout};

/// Given: pretend mod download from a mirror that takes `ms` milliseconds, then answers "done".
pub async fn fetch_jar(ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    "done"
}

/// Some("done") if fetch_jar(work_ms) finishes within limit_ms, otherwise None.
pub async fn jar_with_deadline(work_ms: u64, limit_ms: u64) -> Option<&'static str> {
    // TODO: wait for fetch_jar(work_ms), but give up after limit_ms; turn the Result into an Option
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn jar_with_deadline_keeps_a_fast_mirror() {
        let fast = timeout(Duration::from_secs(3), jar_with_deadline(10, 500)).await.expect("timed out");
        assert_eq!(fast, Some("done"));
    }

    #[tokio::test]
    async fn jar_with_deadline_gives_up_on_a_dead_mirror() {
        let start = Instant::now();
        let slow = timeout(Duration::from_secs(3), jar_with_deadline(2_000, 50)).await.expect("timed out");
        assert_eq!(slow, None);
        assert!(start.elapsed() < Duration::from_millis(1_000), "must give up at the limit, took {:?}", start.elapsed());
    }
}
