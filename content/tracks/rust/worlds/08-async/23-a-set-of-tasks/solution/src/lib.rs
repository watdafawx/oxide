use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

/// Given: pretend download of episode `id` that takes `ms` milliseconds.
pub async fn download(id: u32, ms: u64) -> u32 {
    sleep(Duration::from_millis(ms)).await;
    id
}

/// Downloads every (id, ms) in its own task. Returns the ids in the order the downloads FINISH.
pub async fn finish_order(jobs: Vec<(u32, u64)>) -> Vec<u32> {
    let mut set = JoinSet::new();
    for (id, ms) in jobs {
        set.spawn(download(id, ms));
    }
    let mut order = Vec::new();
    while let Some(result) = set.join_next().await {
        order.push(result.unwrap());
    }
    order
}

/// Starts every download, keeps the first one to finish and cancels the rest.
/// Returns (the first id, how many tasks were cancelled).
pub async fn first_and_cancel_rest(jobs: Vec<(u32, u64)>) -> (Option<u32>, usize) {
    let mut set = JoinSet::new();
    for (id, ms) in jobs {
        set.spawn(download(id, ms));
    }
    let first = match set.join_next().await {
        Some(Ok(id)) => Some(id),
        _ => None,
    };
    set.abort_all();
    let mut cancelled = 0;
    while let Some(result) = set.join_next().await {
        if let Err(e) = result {
            if e.is_cancelled() {
                cancelled += 1;
            }
        }
    }
    (first, cancelled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn finish_order_follows_the_finish_line() {
        let jobs = vec![(1, 300), (2, 20), (3, 150)];
        let order = timeout(Duration::from_secs(5), finish_order(jobs)).await.expect("timed out");
        assert_eq!(order, vec![2, 3, 1], "results must come in the order the downloads finished");
    }

    #[tokio::test]
    async fn the_rest_is_cancelled_after_the_first() {
        let start = Instant::now();
        let jobs = vec![(1, 2_000), (2, 20), (3, 3_000)];
        let out = timeout(Duration::from_secs(5), first_and_cancel_rest(jobs)).await.expect("timed out");
        assert_eq!(out, (Some(2), 2));
        assert!(start.elapsed() < Duration::from_millis(1_000), "the slow downloads must be cancelled, not awaited");
        let none = timeout(Duration::from_secs(5), first_and_cancel_rest(vec![])).await.expect("timed out");
        assert_eq!(none, (None, 0));
    }
}
