use tokio::sync::mpsc;

/// Reads messages until the channel is closed.
pub async fn drain(mut rx: mpsc::Receiver<String>) -> Vec<String> {
    let mut out = Vec::new();
    while let Some(msg) = rx.recv().await {
        out.push(msg);
    }
    out
}

/// Spawns `workers` tasks. Worker `id` (0..workers) sends `id * 10`.
/// Returns everything received, sorted ascending.
pub async fn gather(workers: u32) -> Vec<u32> {
    let (tx, mut rx) = mpsc::channel::<u32>(4);
    for id in 0..workers {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(id * 10).await.unwrap();
        });
    }
    drop(tx);
    let mut out = Vec::new();
    while let Some(v) = rx.recv().await {
        out.push(v);
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn drain_collects_until_closed() {
        let (tx, rx) = mpsc::channel(8);
        tx.send("a".to_string()).await.unwrap();
        tx.send("b".to_string()).await.unwrap();
        tx.send("c".to_string()).await.unwrap();
        drop(tx);
        let out = timeout(Duration::from_secs(2), drain(rx)).await.expect("timed out");
        assert_eq!(out, vec!["a", "b", "c"]);
    }

    #[tokio::test]
    async fn gather_receives_from_all_senders() {
        let out = timeout(Duration::from_secs(2), gather(6)).await.expect("timed out");
        assert_eq!(out, vec![0, 10, 20, 30, 40, 50]);
    }

    #[tokio::test]
    async fn gather_ends_with_zero_workers() {
        let out = timeout(Duration::from_secs(2), gather(0))
            .await
            .expect("gather hung: did you drop the original sender?");
        assert!(out.is_empty());
    }
}
