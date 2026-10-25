use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// Spawns a download worker: every 5 ms it finishes one chunk (add 1 to `chunks`).
/// When `token` is cancelled it returns "stopped".
pub fn spawn_worker(token: CancellationToken, chunks: Arc<AtomicUsize>) -> JoinHandle<&'static str> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token.cancelled() => return "stopped",
                _ = sleep(Duration::from_millis(5)) => {
                    chunks.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
    })
}

/// Cancels the token, waits for every worker, returns how many said "stopped".
pub async fn shutdown(token: CancellationToken, handles: Vec<JoinHandle<&'static str>>) -> usize {
    token.cancel();
    let mut stopped = 0;
    for handle in handles {
        if handle.await.unwrap() == "stopped" {
            stopped += 1;
        }
    }
    stopped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn worker_downloads_then_stops_on_cancel() {
        let token = CancellationToken::new();
        let chunks = Arc::new(AtomicUsize::new(0));
        let handle = spawn_worker(token.clone(), Arc::clone(&chunks));
        sleep(Duration::from_millis(80)).await;
        assert!(chunks.load(Ordering::SeqCst) > 0, "worker never finished a chunk");
        token.cancel();
        let out = timeout(Duration::from_secs(2), handle)
            .await
            .expect("worker did not stop")
            .unwrap();
        assert_eq!(out, "stopped");
        let after = chunks.load(Ordering::SeqCst);
        sleep(Duration::from_millis(40)).await;
        assert_eq!(chunks.load(Ordering::SeqCst), after, "chunks continued after stop");
    }

    #[tokio::test]
    async fn worker_stops_promptly() {
        let token = CancellationToken::new();
        let handle = spawn_worker(token.clone(), Arc::new(AtomicUsize::new(0)));
        sleep(Duration::from_millis(20)).await;
        let start = Instant::now();
        token.cancel();
        timeout(Duration::from_secs(2), handle)
            .await
            .expect("worker did not stop")
            .unwrap();
        assert!(
            start.elapsed() < Duration::from_millis(100),
            "stopped too late: {:?}",
            start.elapsed()
        );
    }

    #[tokio::test]
    async fn shutdown_stops_every_download() {
        let token = CancellationToken::new();
        let chunks = Arc::new(AtomicUsize::new(0));
        let handles: Vec<_> = (0..5)
            .map(|_| spawn_worker(token.clone(), Arc::clone(&chunks)))
            .collect();
        sleep(Duration::from_millis(30)).await;
        let stopped = timeout(Duration::from_secs(2), shutdown(token.clone(), handles))
            .await
            .expect("shutdown hung");
        assert_eq!(stopped, 5);
        assert!(token.is_cancelled());
    }
}
