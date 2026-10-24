//! A tiny threaded TCP echo server (the skeleton of a status server).

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

/// Echoes every byte it reads back to the client until the client closes.
pub fn echo_ping(mut stream: TcpStream) -> io::Result<()> {
    // TODO: read into a buffer in a loop; 0 bytes means the client closed; write_all the bytes back
    todo!()
}

/// Connects to `addr`, sends `msg`, and returns the same number of bytes read back.
pub fn send_ping(addr: SocketAddr, msg: &[u8]) -> io::Result<Vec<u8>> {
    // TODO: connect, write_all(msg), read_exact into a buffer of msg.len()
    todo!()
}

/// Accepts exactly `connections` clients. Each runs `echo_ping` on its own thread.
/// Returns after all of them are finished.
pub fn serve(listener: TcpListener, connections: usize) {
    // TODO: accept in a loop, run echo_ping for each client on its own thread, join all at the end
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    const TIMEOUT: Duration = Duration::from_secs(3);

    fn connect(addr: SocketAddr) -> TcpStream {
        let s = TcpStream::connect_timeout(&addr, TIMEOUT).expect("connect to test server");
        s.set_read_timeout(Some(TIMEOUT)).unwrap();
        s
    }

    #[test]
    fn echo_ping_answers_until_the_client_hangs_up() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let _ = echo_ping(stream);
        });

        let mut c = connect(addr);
        c.write_all(b"hello ").unwrap();
        c.write_all(b"world").unwrap();
        c.shutdown(std::net::Shutdown::Write).unwrap(); // tell the server we're done
        let mut got = Vec::new();
        let _ = c.read_to_end(&mut got);
        assert_eq!(got, b"hello world");
    }

    #[test]
    fn send_ping_round_trips_through_the_server() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        thread::spawn(move || serve(listener, 1));

        let got = send_ping(addr, b"ping 123").expect("round trip");
        assert_eq!(got, b"ping 123");
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
            .expect("second client should be answered while the first is idle");
        assert_eq!(&got, b"bee");

        a.write_all(b"ay").unwrap();
        let mut got = [0u8; 2];
        a.read_exact(&mut got).expect("first client should be answered too");
        assert_eq!(&got, b"ay");
    }
}
