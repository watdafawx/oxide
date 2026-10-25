use std::sync::mpsc::{self, Receiver};
use std::thread;

/// Starts a thread that sends every file name (in order) and returns the receiving end.
pub fn send_all(names: Vec<String>) -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for name in names {
            tx.send(name).unwrap();
        }
    });
    rx
}

/// Collects everything from `rx` until the channel closes.
pub fn drain(rx: Receiver<u64>) -> Vec<u64> {
    let mut got = Vec::new();
    for size in rx {
        got.push(size);
    }
    got
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::RecvTimeoutError;
    use std::time::Duration;

    const WAIT: Duration = Duration::from_secs(5);

    #[test]
    fn send_all_delivers_in_order() {
        let rx = send_all(vec!["a.txt".into(), "b.txt".into(), "c.txt".into()]);
        let mut got = Vec::new();
        for _ in 0..3 {
            got.push(rx.recv_timeout(WAIT).expect("expected another name"));
        }
        assert_eq!(got, vec!["a.txt", "b.txt", "c.txt"]);
        assert_eq!(
            rx.recv_timeout(WAIT),
            Err(RecvTimeoutError::Disconnected),
            "the channel should close after the last name"
        );
    }

    #[test]
    fn drain_collects_until_the_sender_hangs_up() {
        let (tx, rx) = mpsc::channel();
        for size in [5, 6, 7] {
            tx.send(size).unwrap();
        }
        drop(tx);
        assert_eq!(drain(rx), vec![5, 6, 7]);
    }
}
