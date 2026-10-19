//! A tiny UDP request/response pair (a status query).

use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

/// Receives one datagram on `socket` and sends it back, uppercased, to the sender.
pub fn answer_query(socket: &UdpSocket) -> io::Result<()> {
    let mut buf = [0u8; 512];
    let (n, from) = socket.recv_from(&mut buf)?;
    socket.send_to(&buf[..n].to_ascii_uppercase(), from)?;
    Ok(())
}

/// Sends `msg` to `server` from a fresh socket and returns the reply.
/// Gives up with an `Err` after 500 ms of silence.
pub fn send_query(server: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;
    socket.send_to(msg, server)?;
    let mut buf = [0u8; 512];
    let (n, _from) = socket.recv_from(&mut buf)?;
    Ok(buf[..n].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn bound() -> (UdpSocket, SocketAddr) {
        let s = UdpSocket::bind("127.0.0.1:0").unwrap();
        s.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        let addr = s.local_addr().unwrap();
        (s, addr)
    }

    #[test]
    fn answer_query_replies_in_uppercase() {
        let (server, server_addr) = bound();
        let (client, _) = bound();
        client.send_to(b"hello udp", server_addr).unwrap();

        let t = thread::spawn(move || answer_query(&server));

        let mut buf = [0u8; 64];
        let (n, _) = client.recv_from(&mut buf).expect("no reply from answer_query");
        assert_eq!(&buf[..n], b"HELLO UDP");
        t.join().unwrap().unwrap();
    }

    #[test]
    fn send_query_gets_a_reply() {
        let (server, server_addr) = bound();
        let t = thread::spawn(move || answer_query(&server));

        let reply = send_query(server_addr, b"ping").expect("query failed");
        assert_eq!(reply, b"PING");
        t.join().unwrap().unwrap();
    }

    #[test]
    fn send_query_gives_up_when_nobody_answers() {
        // Bound but never read: the datagram is accepted and never answered.
        let (_silent, silent_addr) = bound();
        let started = std::time::Instant::now();
        let result = send_query(silent_addr, b"anyone?");
        assert!(result.is_err(), "no reply should be an Err, got {result:?}");
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "send_query should give up after about 500 ms"
        );
    }
}
