use std::time::Duration;
use tokio::time::sleep;

/// Given: pretend request to one mod mirror that answers after `ms` milliseconds.
pub async fn ping_mirror(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

/// Asks mirror A and mirror B at the same time.
/// Returns "mirror-a" or "mirror-b", whichever answers first.
pub async fn fastest_mirror(a_ms: u64, b_ms: u64) -> &'static str {
    tokio::select! {
        _ = ping_mirror(a_ms) => "mirror-a",
        _ = ping_mirror(b_ms) => "mirror-b",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn fastest_mirror_picks_the_quicker_one() {
        let a = timeout(Duration::from_secs(3), fastest_mirror(10, 800)).await.expect("timed out");
        assert_eq!(a, "mirror-a");
        let b = timeout(Duration::from_secs(3), fastest_mirror(800, 10)).await.expect("timed out");
        assert_eq!(b, "mirror-b");
    }

    #[tokio::test]
    async fn fastest_mirror_does_not_wait_for_the_slow_one() {
        let start = Instant::now();
        timeout(Duration::from_secs(3), fastest_mirror(20, 2_000)).await.expect("timed out");
        assert!(start.elapsed() < Duration::from_millis(500), "waited for the slow mirror too: {:?}", start.elapsed());
    }
}
