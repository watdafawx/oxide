//! A tiny UDP request and answer.

use std::io;
use std::net::{SocketAddr, UdpSocket};

/// Receives one datagram on `socket` and sends it back, uppercased, to whoever sent it.
pub fn serve_once(socket: &UdpSocket) -> io::Result<()> {
    // TODO: receive one datagram (and who sent it), then send its bytes back
    //       in UPPERCASE to that sender
    todo!()
}

/// Sends `msg` to `server` from a fresh socket and returns the reply.
pub fn request(server: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    // TODO: bind a fresh socket on 127.0.0.1, any free port; send msg to the
    //       server; receive one datagram and give back just its bytes
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

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

        let reply = request(server_addr, b"ping").expect("no reply");
        assert_eq!(reply, b"PING", "only the bytes that arrived, not the whole buffer");
        t.join().unwrap().unwrap();
    }
}
