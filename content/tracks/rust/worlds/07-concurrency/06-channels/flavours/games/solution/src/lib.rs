use std::sync::mpsc::{self, Receiver};
use std::thread;

/// Starts a thread that sends every jar name (in folder order) and returns the receiving end.
pub fn stream_jars(jars: Vec<String>) -> Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for jar in jars {
            tx.send(jar).unwrap();
        }
    });
    rx
}

/// Collects every jar size from `rx` until the channel closes.
pub fn drain_sizes(rx: Receiver<u64>) -> Vec<u64> {
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
    fn stream_jars_delivers_in_folder_order() {
        let rx = stream_jars(vec!["sodium.jar".into(), "iris.jar".into(), "jei.jar".into()]);
        let mut got = Vec::new();
        for _ in 0..3 {
            got.push(rx.recv_timeout(WAIT).expect("expected another jar name"));
        }
        assert_eq!(got, vec!["sodium.jar", "iris.jar", "jei.jar"]);
        assert_eq!(
            rx.recv_timeout(WAIT),
            Err(RecvTimeoutError::Disconnected),
            "the channel should close after the last jar"
        );
    }

    #[test]
    fn drain_sizes_collects_until_the_sender_hangs_up() {
        let (tx, rx) = mpsc::channel();
        for size in [5, 6, 7] {
            tx.send(size).unwrap();
        }
        drop(tx);
        assert_eq!(drain_sizes(rx), vec![5, 6, 7]);
    }
}
