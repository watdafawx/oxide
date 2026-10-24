use std::sync::mpsc::{self, Receiver};
use std::thread;

/// Starts `workers` hash workers; worker `id` (0-based) reports `id * 10` once.
/// The returned Receiver must close after the last worker finishes.
pub fn hash_workers(workers: u32) -> Receiver<u32> {
    let (tx, rx) = mpsc::channel();
    for id in 0..workers {
        let tx = tx.clone();
        thread::spawn(move || tx.send(id * 10).unwrap());
    }
    drop(tx);
    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

    #[test]
    fn hash_workers_all_report_and_the_channel_closes() {
        let rx = hash_workers(4);
        let mut got = Vec::new();
        let end = loop {
            match rx.recv_timeout(Duration::from_secs(5)) {
                Ok(v) => got.push(v),
                Err(e) => break e,
            }
        };
        got.sort();
        assert_eq!(got, vec![0, 10, 20, 30], "one report from every worker");
        assert_eq!(
            end,
            RecvTimeoutError::Disconnected,
            "the channel never closed: did you drop the original Sender?"
        );
    }
}
