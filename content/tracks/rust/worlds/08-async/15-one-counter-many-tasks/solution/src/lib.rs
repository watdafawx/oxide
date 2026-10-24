use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::yield_now;

/// Adds one to the "episodes done" counter: read it, wait (other tasks run), write it back plus one.
pub async fn record_done(done: &Mutex<usize>) {
    let mut guard = done.lock().await;
    let current = *guard;
    yield_now().await; // stands for any wait between reading and writing
    *guard = current + 1;
}

/// Spawns `tasks` tasks that each call record_done `per_task` times. Returns the final count.
pub async fn finish_all(tasks: usize, per_task: usize) -> usize {
    let done = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();
    for _ in 0..tasks {
        let done = Arc::clone(&done);
        handles.push(tokio::spawn(async move {
            for _ in 0..per_task {
                record_done(&done).await;
            }
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    *done.lock().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn record_done_adds_one() {
        let done = Mutex::new(0usize);
        timeout(Duration::from_secs(2), async {
            record_done(&done).await;
            record_done(&done).await;
            record_done(&done).await;
        })
        .await
        .expect("timed out");
        assert_eq!(*done.lock().await, 3);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn finish_all_loses_no_updates() {
        let total = timeout(Duration::from_secs(5), finish_all(10, 20)).await.expect("timed out");
        assert_eq!(total, 200, "updates were lost: hold the lock over the whole read, wait, write");
    }
}
