//! A TCP ping: connect to a server, send, read the answer.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};

/// Connects to `addr`, sends all of `msg`, and reads back exactly as many
/// bytes as it sent.
pub fn send_ping(addr: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(msg)?;
    let mut buf = vec![0u8; msg.len()];
    stream.read_exact(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    /// A test server: reads `len` bytes, then sends them back one byte at a time.
    fn server(len: usize, in_pieces: bool) -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            s.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            let mut buf = vec![0u8; len];
            if s.read_exact(&mut buf).is_err() {
                return;
            }
            if in_pieces {
                for b in buf {
                    let _ = s.write_all(&[b]);
                    thread::sleep(Duration::from_millis(20));
                }
            } else {
                let _ = s.write_all(&buf);
            }
        });
        addr
    }

    #[test]
    fn send_ping_round_trips_through_the_server() {
        let addr = server(8, false);
        let got = send_ping(addr, b"ping 123").expect("round trip");
        assert_eq!(got, b"ping 123");
    }

    #[test]
    fn slow_answer_is_read_whole() {
        let addr = server(5, true);
        let got = send_ping(addr, b"hello").expect("round trip");
        assert_eq!(got, b"hello", "the answer came in 5 pieces; all of them must be read");
    }
}
