use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::yield_now;

/// Adds one to the "jars installed" counter: read it, wait (other tasks run), write it back plus one.
pub async fn record_installed(installed: &Mutex<usize>) {
    let mut guard = installed.lock().await;
    let current = *guard;
    yield_now().await; // stands for any wait between reading and writing
    *guard = current + 1;
}

/// Spawns `tasks` download tasks that each call record_installed `per_task` times. Returns the final count.
pub async fn install_all(tasks: usize, per_task: usize) -> usize {
    let installed = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();
    for _ in 0..tasks {
        let installed = Arc::clone(&installed);
        handles.push(tokio::spawn(async move {
            for _ in 0..per_task {
                record_installed(&installed).await;
            }
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    *installed.lock().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn record_installed_adds_one() {
        let installed = Mutex::new(0usize);
        timeout(Duration::from_secs(2), async {
            record_installed(&installed).await;
            record_installed(&installed).await;
            record_installed(&installed).await;
        })
        .await
        .expect("timed out");
        assert_eq!(*installed.lock().await, 3);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn install_all_loses_no_updates() {
        let total = timeout(Duration::from_secs(5), install_all(10, 20)).await.expect("timed out");
        assert_eq!(total, 200, "updates were lost: hold the lock over the whole read, wait, write");
    }
}
