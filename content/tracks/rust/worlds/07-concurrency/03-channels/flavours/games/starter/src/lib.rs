use std::sync::mpsc::{self, Receiver};
use std::thread;

/// What a worker reports back: which worker did the job and the hash it found.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HashResult {
    pub worker: u32,
    pub hash: u32,
}

/// Spawns a thread that sends every jar name (in order), and returns the receiving end.
pub fn stream_names(names: Vec<String>) -> Receiver<String> {
    // TODO: channel, move tx + names into a thread, send each name, return rx
    todo!()
}

/// Spawns `workers` threads; worker `id` (0-based) sends one
/// `HashResult { worker: id, hash: id * 10 }`.
/// The returned Receiver must disconnect after the last worker finishes.
pub fn hash_workers(workers: u32) -> Receiver<HashResult> {
    // TODO: one tx.clone() per worker; do not forget to drop the original tx
    todo!()
}

/// Collects everything from `rx` until all senders are gone.
pub fn drain_results(rx: Receiver<HashResult>) -> Vec<HashResult> {
    // TODO: iterate the receiver
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

    const WAIT: Duration = Duration::from_secs(5);

    #[test]
    fn stream_names_delivers_every_jar_in_order() {
        let rx = stream_names(vec!["sodium.jar".into(), "iris.jar".into(), "jei.jar".into()]);
        let mut got = Vec::new();
        for _ in 0..3 {
            got.push(rx.recv_timeout(WAIT).expect("expected another jar name"));
        }
        assert_eq!(got, vec!["sodium.jar", "iris.jar", "jei.jar"]);
        assert_eq!(
            rx.recv_timeout(WAIT),
            Err(RecvTimeoutError::Disconnected),
            "the sender should be dropped after the last name"
        );
    }

    #[test]
    fn hash_workers_report_and_the_channel_closes() {
        let rx = hash_workers(4);
        let mut got = Vec::new();
        let end = loop {
            match rx.recv_timeout(WAIT) {
                Ok(v) => got.push(v),
                Err(e) => break e,
            }
        };
        got.sort();
        let expected: Vec<HashResult> = (0..4).map(|id| HashResult { worker: id, hash: id * 10 }).collect();
        assert_eq!(got, expected);
        assert_eq!(
            end,
            RecvTimeoutError::Disconnected,
            "channel never closed: did you drop the original Sender?"
        );
    }

    #[test]
    fn drain_results_collects_until_workers_hang_up() {
        let (tx, rx) = mpsc::channel();
        for n in [5, 6, 7] {
            tx.send(HashResult { worker: n, hash: n * 2 }).unwrap();
        }
        drop(tx);
        let got = drain_results(rx);
        assert_eq!(got.len(), 3);
        assert_eq!(got[2], HashResult { worker: 7, hash: 14 });
    }
}
