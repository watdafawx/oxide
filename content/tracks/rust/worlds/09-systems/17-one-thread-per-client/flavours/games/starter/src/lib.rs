//! A server that answers many players at once.

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Writes back every byte it reads, until the other side closes the connection.
/// (Your solution from the last quest.)
pub fn echo_ping(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..n])?;
    }
}

/// Accepts exactly `connections` clients and runs `echo_ping` for each of them.
/// Returns after all of them are finished.
pub fn serve(listener: TcpListener, connections: usize) {
    // TODO: this serves the clients one after another, so one quiet client
    //       blocks everyone behind it. Give each accepted connection its own
    //       thread, keep the handles, and wait for all of them at the end.
    for _ in 0..connections {
        match listener.accept() {
            Ok((stream, _peer)) => {
                let _ = echo_ping(stream);
            }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::time::Duration;

    fn connect(addr: SocketAddr) -> TcpStream {
        let s = TcpStream::connect_timeout(&addr, Duration::from_secs(3)).expect("connect to test server");
        s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
        s
    }

    #[test]
    fn serves_two_players_at_once() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || serve(listener, 2));

        let mut a = connect(addr); // connects first, stays silent
        let mut b = connect(addr);

        // b talks while a is idle. A server that handles clients one at a
        // time is still stuck on `a`, so this read times out.
        b.write_all(b"bee").unwrap();
        let mut got = [0u8; 3];
        b.read_exact(&mut got)
            .expect("the second client should be answered while the first is quiet");
        assert_eq!(&got, b"bee");

        a.write_all(b"ay").unwrap();
        let mut got = [0u8; 2];
        a.read_exact(&mut got).expect("the first client should be answered too");
        assert_eq!(&got, b"ay");
    }
}
