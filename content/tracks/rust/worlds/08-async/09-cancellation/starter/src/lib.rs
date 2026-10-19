use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// Spawns a task: every 5 ms add 1 to `ticks`. When `token` is cancelled, return "stopped".
pub fn spawn_worker(token: CancellationToken, ticks: Arc<AtomicUsize>) -> JoinHandle<&'static str> {
    // TODO: tokio::spawn(async move { loop { select! { cancelled => return "stopped", sleep => tick } } })
    todo!()
}

/// Cancels the token, waits for every worker, returns how many said "stopped".
pub async fn shutdown(token: CancellationToken, handles: Vec<JoinHandle<&'static str>>) -> usize {
    // TODO: token.cancel(), then await each handle and count "stopped"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn worker_ticks_then_stops_on_cancel() {
        let token = CancellationToken::new();
        let ticks = Arc::new(AtomicUsize::new(0));
        let handle = spawn_worker(token.clone(), Arc::clone(&ticks));
        sleep(Duration::from_millis(80)).await;
        assert!(ticks.load(Ordering::SeqCst) > 0, "worker never ticked");
        token.cancel();
        let out = timeout(Duration::from_secs(2), handle).await.expect("worker did not stop").unwrap();
        assert_eq!(out, "stopped");
        let after = ticks.load(Ordering::SeqCst);
        sleep(Duration::from_millis(40)).await;
        assert_eq!(ticks.load(Ordering::SeqCst), after, "ticks continued after stop");
    }

    #[tokio::test]
    async fn worker_stops_promptly() {
        let token = CancellationToken::new();
        let handle = spawn_worker(token.clone(), Arc::new(AtomicUsize::new(0)));
        sleep(Duration::from_millis(20)).await;
        let start = Instant::now();
        token.cancel();
        timeout(Duration::from_secs(2), handle).await.expect("worker did not stop").unwrap();
        assert!(start.elapsed() < Duration::from_millis(100), "stopped too late: {:?}", start.elapsed());
    }

    #[tokio::test]
    async fn shutdown_stops_every_worker() {
        let token = CancellationToken::new();
        let ticks = Arc::new(AtomicUsize::new(0));
        let handles: Vec<_> = (0..5)
            .map(|_| spawn_worker(token.clone(), Arc::clone(&ticks)))
            .collect();
        sleep(Duration::from_millis(30)).await;
        let stopped = timeout(Duration::from_secs(2), shutdown(token.clone(), handles))
            .await
            .expect("shutdown hung");
        assert_eq!(stopped, 5);
        assert!(token.is_cancelled());
    }
}
