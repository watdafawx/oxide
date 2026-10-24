use std::time::Duration;
use tokio::time::sleep;

/// Given: pretend request to one server that answers after `ms` milliseconds.
pub async fn ask(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

/// Asks the main server and the backup server at the same time.
/// Returns "main" or "backup", whichever answers first.
pub async fn first_answer(main_ms: u64, backup_ms: u64) -> &'static str {
    // TODO: race ask(main_ms) against ask(backup_ms) and name the one that finished first
    let _ = (main_ms, backup_ms);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn first_answer_picks_the_faster_server() {
        let a = timeout(Duration::from_secs(3), first_answer(10, 800)).await.expect("timed out");
        assert_eq!(a, "main");
        let b = timeout(Duration::from_secs(3), first_answer(800, 10)).await.expect("timed out");
        assert_eq!(b, "backup");
    }

    #[tokio::test]
    async fn first_answer_does_not_wait_for_the_loser() {
        let start = Instant::now();
        timeout(Duration::from_secs(3), first_answer(20, 2_000)).await.expect("timed out");
        assert!(start.elapsed() < Duration::from_millis(500), "waited for the slow server too: {:?}", start.elapsed());
    }
}
