//! A UDP status query that gives up when the server is gone.

use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

/// Sends `msg` to `server` from a fresh socket and returns the reply.
/// Gives up with an `Err` after 500 ms of silence.
pub fn send_query(server: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    // TODO: a lost datagram makes recv_from wait forever. Give the socket a
    //       read deadline of 500 milliseconds before receiving.
    let _wait: Option<Duration> = None;
    socket.send_to(msg, server)?;
    let mut buf = [0u8; 512];
    let (n, _from) = socket.recv_from(&mut buf)?;
    Ok(buf[..n].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn send_query_gives_up_when_nobody_answers() {
        // Bound but never read: the datagram is accepted and never answered.
        let silent = UdpSocket::bind("127.0.0.1:0").unwrap();
        let silent_addr = silent.local_addr().unwrap();

        // Run it on a helper thread so a request that waits forever can't hang the test.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(send_query(silent_addr, b"anyone?"));
        });
        match rx.recv_timeout(Duration::from_secs(2)) {
            Ok(result) => assert!(result.is_err(), "no reply should be an Err, got {result:?}"),
            Err(_) => panic!("still waiting after 2 s: give the socket a read timeout of 500 ms"),
        }
        drop(silent);
    }
}
