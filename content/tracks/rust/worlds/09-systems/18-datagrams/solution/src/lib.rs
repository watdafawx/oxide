//! A tiny UDP request and answer.

use std::io;
use std::net::{SocketAddr, UdpSocket};

/// Receives one datagram on `socket` and sends it back, uppercased, to whoever sent it.
pub fn serve_once(socket: &UdpSocket) -> io::Result<()> {
    let mut buf = [0u8; 512];
    let (n, from) = socket.recv_from(&mut buf)?;
    socket.send_to(&buf[..n].to_ascii_uppercase(), from)?;
    Ok(())
}

/// Sends `msg` to `server` from a fresh socket and returns the reply.
pub fn request(server: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.send_to(msg, server)?;
    let mut buf = [0u8; 512];
    let (n, _from) = socket.recv_from(&mut buf)?;
    Ok(buf[..n].to_vec())
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
