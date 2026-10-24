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
    // TODO: spawn every download into one JoinSet
    // TODO: take results out of the set as they finish, and collect the ids in that order
    let _ = (jobs, JoinSet::<u32>::new());
    todo!()
}

/// Starts every download, keeps the first one to finish and cancels the rest.
/// Returns (the first id, how many tasks were cancelled).
pub async fn first_and_cancel_rest(jobs: Vec<(u32, u64)>) -> (Option<u32>, usize) {
    // TODO: spawn every download into one JoinSet, and wait for the first one to finish
    // TODO: cancel all the others, then take their results out and count the cancelled ones
    let _ = jobs;
    todo!()
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
