use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::yield_now;

/// Adds one to the "episodes done" counter: read it, wait (other tasks run), write it back plus one.
pub async fn record_done(done: &Mutex<usize>) {
    // TODO: lock the counter and keep the guard; read the value,
    // call yield_now().await (it stands for any wait in between), then write the value plus one
    let _ = done;
    yield_now().await;
    todo!()
}

/// Spawns `tasks` tasks that each call record_done `per_task` times. Returns the final count.
pub async fn finish_all(tasks: usize, per_task: usize) -> usize {
    // TODO: put a counter starting at 0 in an Arc<Mutex<usize>>
    // TODO: spawn `tasks` tasks; each gets its own Arc clone and calls record_done per_task times
    // TODO: await every handle, then lock the counter once more and return its value
    let _ = (tasks, per_task, Arc::new(()));
    todo!()
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
