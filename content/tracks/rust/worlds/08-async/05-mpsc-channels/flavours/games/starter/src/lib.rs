use tokio::sync::mpsc;

/// Reads progress lines until the channel is closed.
pub async fn drain_log(mut rx: mpsc::Receiver<String>) -> Vec<String> {
    // TODO: keep receiving with a while let loop and push each line; stop when recv() gives None
    todo!()
}

/// Spawns `downloads` tasks. Download `id` (0..downloads) sends `id * 10` (percent).
/// Returns everything received, sorted ascending.
pub async fn gather_progress(downloads: u32) -> Vec<u32> {
    // TODO: channel, one tx.clone() per task, drop the original tx, collect, sort
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn drain_log_collects_until_closed() {
        let (tx, rx) = mpsc::channel(8);
        tx.send("sodium.jar 10%".to_string()).await.unwrap();
        tx.send("sodium.jar 60%".to_string()).await.unwrap();
        tx.send("sodium.jar done".to_string()).await.unwrap();
        drop(tx);
        let out = timeout(Duration::from_secs(2), drain_log(rx)).await.expect("timed out");
        assert_eq!(out, vec!["sodium.jar 10%", "sodium.jar 60%", "sodium.jar done"]);
    }

    #[tokio::test]
    async fn gather_progress_hears_every_download() {
        let out = timeout(Duration::from_secs(2), gather_progress(6)).await.expect("timed out");
        assert_eq!(out, vec![0, 10, 20, 30, 40, 50]);
    }

    #[tokio::test]
    async fn gather_progress_ends_with_no_downloads() {
        let out = timeout(Duration::from_secs(2), gather_progress(0))
            .await
            .expect("gather_progress hung: did you drop the original sender?");
        assert!(out.is_empty());
    }
}
