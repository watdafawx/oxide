use std::sync::mpsc::{self, Receiver};
use std::thread;

/// Spawns a thread that sends every item (in order), and returns the receiving end.
pub fn send_all(items: Vec<String>) -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for item in items {
            tx.send(item).unwrap();
        }
    });
    rx
}

/// Spawns `workers` threads; worker `id` (0-based) sends `id * 10` once.
/// The returned Receiver must disconnect after the last worker finishes.
pub fn fan_out(workers: u32) -> Receiver<u32> {
    let (tx, rx) = mpsc::channel();
    for id in 0..workers {
        let tx = tx.clone();
        thread::spawn(move || tx.send(id * 10).unwrap());
    }
    drop(tx);
    rx
}

/// Collects everything from `rx` until all senders are gone.
pub fn drain(rx: Receiver<u32>) -> Vec<u32> {
    rx.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

    const WAIT: Duration = Duration::from_secs(5);

    #[test]
    fn send_all_delivers_in_order() {
        let rx = send_all(vec!["a".into(), "b".into(), "c".into()]);
        let mut got = Vec::new();
        for _ in 0..3 {
            got.push(rx.recv_timeout(WAIT).expect("expected another item"));
        }
        assert_eq!(got, vec!["a", "b", "c"]);
        assert_eq!(
            rx.recv_timeout(WAIT),
            Err(RecvTimeoutError::Disconnected),
            "the sender should be dropped after the last item"
        );
    }

    #[test]
    fn fan_out_delivers_from_every_worker() {
        let rx = fan_out(4);
        let mut got = Vec::new();
        let end = loop {
            match rx.recv_timeout(WAIT) {
                Ok(v) => got.push(v),
                Err(e) => break e,
            }
        };
        got.sort();
        assert_eq!(got, vec![0, 10, 20, 30]);
        assert_eq!(
            end,
            RecvTimeoutError::Disconnected,
            "channel never closed: did you drop the original Sender?"
        );
    }

    #[test]
    fn drain_collects_until_senders_hang_up() {
        let (tx, rx) = mpsc::channel();
        for n in [5, 6, 7] {
            tx.send(n).unwrap();
        }
        drop(tx);
        assert_eq!(drain(rx), vec![5, 6, 7]);
    }
}
