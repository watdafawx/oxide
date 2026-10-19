//! A tiny UDP request/response pair.

use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

/// Receives one datagram on `socket` and sends it back, uppercased, to the sender.
pub fn serve_once(socket: &UdpSocket) -> io::Result<()> {
    // TODO: recv_from into a buffer -> (n, from); then send_to(&buf[..n].to_ascii_uppercase(), from)
    todo!()
}

/// Sends `msg` to `server` from a fresh socket and returns the reply.
/// Gives up with an `Err` after 500 ms of silence.
pub fn request(server: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    // TODO: bind 127.0.0.1:0, set_read_timeout(Some(Duration::from_millis(500))),
    //       send_to(msg, server), recv_from into a buffer, return buf[..n].to_vec()
    todo!()
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
    fn server_replies_uppercase() {
        let (server, server_addr) = bound();
        let (client, _) = bound();
        client.send_to(b"hello udp", server_addr).unwrap();

        let t = thread::spawn(move || serve_once(&server));

        let mut buf = [0u8; 64];
        let (n, _) = client.recv_from(&mut buf).expect("no reply from serve_once");
        assert_eq!(&buf[..n], b"HELLO UDP");
        t.join().unwrap().unwrap();
    }

    #[test]
    fn request_gets_reply() {
        let (server, server_addr) = bound();
        let t = thread::spawn(move || serve_once(&server));

        let reply = request(server_addr, b"ping").expect("request failed");
        assert_eq!(reply, b"PING");
        t.join().unwrap().unwrap();
    }

    #[test]
    fn request_times_out_when_nobody_answers() {
        // Bound but never read: the datagram is accepted and never answered.
        let (_silent, silent_addr) = bound();
        let started = std::time::Instant::now();
        let result = request(silent_addr, b"anyone?");
        assert!(result.is_err(), "no reply should be an Err, got {result:?}");
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "request should give up after about 500 ms"
        );
    }
}
