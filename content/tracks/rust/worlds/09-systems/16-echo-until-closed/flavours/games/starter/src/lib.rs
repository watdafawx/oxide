//! The server side of a ping: echo until the client hangs up.

use std::io::{self, Read, Write};
use std::net::TcpStream;

/// Writes back every byte it reads, until the other side closes the connection.
pub fn echo_ping(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    // TODO: again and again: read into buf; 0 bytes means the client has
    //       closed, so stop with Ok(()); otherwise write back exactly the
    //       bytes that arrived
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Shutdown, SocketAddr, TcpListener};
    use std::thread;
    use std::time::Duration;

    fn start_server() -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let _ = echo_ping(stream);
        });
        addr
    }

    /// Sends `parts`, says "done", and returns everything the server sent back.
    fn talk(parts: &[&[u8]]) -> Vec<u8> {
        let mut c = TcpStream::connect(start_server()).unwrap();
        c.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        for p in parts {
            c.write_all(p).unwrap();
        }
        c.shutdown(Shutdown::Write).unwrap(); // tell the server we are done
        let mut got = Vec::new();
        let _ = c.read_to_end(&mut got);
        got
    }

    #[test]
    fn echo_ping_answers_until_the_client_hangs_up() {
        assert_eq!(talk(&[b"hello ", b"world"]), b"hello world");
        assert_eq!(talk(&[]), b"", "a client that says nothing gets nothing");
    }

    #[test]
    fn echoes_a_big_packet() {
        let big = vec![7u8; 3000];
        assert_eq!(talk(&[&big]).len(), 3000, "3000 bytes do not fit in one 1024-byte read");
    }
}
