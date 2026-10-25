use std::sync::mpsc::{self, Receiver};
use std::thread;

/// Starts `workers` threads; worker `id` (0-based) sends `id * 10` once.
/// The returned Receiver must close after the last worker finishes.
pub fn fan_out(workers: u32) -> Receiver<u32> {
    let (tx, rx) = mpsc::channel();
    // TODO: for each worker id, clone the Sender and move the clone into a thread
    // that sends id * 10; then drop the original Sender
    let _ = (workers, tx);
    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

    #[test]
    fn fan_out_hears_every_worker_and_closes() {
        let rx = fan_out(4);
        let mut got = Vec::new();
        let end = loop {
            match rx.recv_timeout(Duration::from_secs(5)) {
                Ok(v) => got.push(v),
                Err(e) => break e,
            }
        };
        got.sort();
        assert_eq!(got, vec![0, 10, 20, 30], "one value from every worker");
        assert_eq!(
            end,
            RecvTimeoutError::Disconnected,
            "the channel never closed: did you drop the original Sender?"
        );
    }
}
