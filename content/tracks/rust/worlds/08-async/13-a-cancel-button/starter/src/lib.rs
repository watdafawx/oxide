use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::sleep;

/// Given: pretend download that takes `ms` milliseconds, then answers "done".
pub async fn download(ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    "done"
}

/// Runs download(work_ms). If the Cancel button (`cancel`) is pressed first, stop and return "cancelled".
pub async fn download_or_cancel(work_ms: u64, mut cancel: oneshot::Receiver<()>) -> &'static str {
    // TODO: race the cancel receiver (borrowed) against download(work_ms);
    // "cancelled" if the button wins, otherwise the download's own answer
    let _ = (work_ms, &mut cancel);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    #[tokio::test]
    async fn cancel_stops_the_download() {
        let (press, cancel) = oneshot::channel();
        let task = tokio::spawn(download_or_cancel(5_000, cancel));
        sleep(Duration::from_millis(30)).await;
        press.send(()).unwrap();
        let out = timeout(Duration::from_secs(2), task).await.expect("did not stop after Cancel").unwrap();
        assert_eq!(out, "cancelled");
    }

    #[tokio::test]
    async fn no_cancel_lets_it_finish() {
        let (_press, cancel) = oneshot::channel::<()>(); // the button exists but is never pressed
        let out = timeout(Duration::from_secs(2), download_or_cancel(20, cancel)).await.expect("timed out");
        assert_eq!(out, "done");
    }
}
