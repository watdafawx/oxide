use tokio::task::JoinHandle;

/// Starts a background task that computes n * n.
pub fn spawn_square(n: u64) -> JoinHandle<u64> {
    // TODO: tokio::spawn(async move { ... })
    todo!()
}

/// Squares every number in its own task. Results keep the input order.
pub async fn square_all(nums: Vec<u64>) -> Vec<u64> {
    // TODO: spawn all the tasks first (collect the handles), then await each handle
    todo!()
}

/// Divides in a spawned task. Division by zero panics, so give back None then.
pub async fn safe_div(a: i32, b: i32) -> Option<i32> {
    // TODO: spawn the division, await the handle, turn a JoinError into None with `.ok()`
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    // spawn_square is not async, but spawn needs a runtime: #[tokio::test] provides one.
    #[tokio::test]
    async fn spawn_square_returns_a_handle() {
        let handle = spawn_square(7);
        let out = timeout(Duration::from_secs(5), handle).await.expect("timed out");
        assert_eq!(out.unwrap(), 49);
    }

    #[tokio::test]
    async fn square_all_keeps_order() {
        let out = timeout(Duration::from_secs(5), square_all(vec![1, 2, 3, 4, 10]))
            .await
            .expect("timed out");
        assert_eq!(out, vec![1, 4, 9, 16, 100]);
    }

    #[tokio::test]
    async fn safe_div_survives_a_panic() {
        let ok = timeout(Duration::from_secs(5), safe_div(10, 2)).await.expect("timed out");
        assert_eq!(ok, Some(5));
        let bad = timeout(Duration::from_secs(5), safe_div(1, 0)).await.expect("timed out");
        assert_eq!(bad, None);
    }
}
