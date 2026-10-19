use tokio::task::JoinHandle;

/// Starts a background task that turns a size in KB into bytes (kb * 1024).
pub fn spawn_fetch(kb: u64) -> JoinHandle<u64> {
    tokio::spawn(async move { kb * 1024 })
}

/// Fetches every file in its own task. Results keep the queue order.
pub async fn fetch_all(sizes_kb: Vec<u64>) -> Vec<u64> {
    let handles: Vec<_> = sizes_kb.into_iter().map(spawn_fetch).collect();
    let mut out = Vec::with_capacity(handles.len());
    for handle in handles {
        out.push(handle.await.unwrap());
    }
    out
}

/// Average file size, computed in a spawned task. With zero files the division
/// panics inside the task, so give back None.
pub async fn average_kb(total_kb: i32, files: i32) -> Option<i32> {
    tokio::spawn(async move { total_kb / files }).await.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    // spawn_fetch is not async, but spawn needs a runtime: #[tokio::test] provides one.
    #[tokio::test]
    async fn spawn_fetch_returns_a_handle() {
        let handle = spawn_fetch(2);
        let out = timeout(Duration::from_secs(5), handle).await.expect("timed out");
        assert_eq!(out.unwrap(), 2_048);
    }

    #[tokio::test]
    async fn fetch_all_keeps_the_queue_order() {
        let out = timeout(Duration::from_secs(5), fetch_all(vec![1, 2, 3, 10]))
            .await
            .expect("timed out");
        assert_eq!(out, vec![1_024, 2_048, 3_072, 10_240]);
    }

    #[tokio::test]
    async fn average_kb_survives_a_panic() {
        let ok = timeout(Duration::from_secs(5), average_kb(900, 3)).await.expect("timed out");
        assert_eq!(ok, Some(300));
        let bad = timeout(Duration::from_secs(5), average_kb(1, 0)).await.expect("timed out");
        assert_eq!(bad, None);
    }
}
