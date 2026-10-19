**Why now:** a Minecraft server listens on a TCP port and answers each connection. Before speaking its real protocol, build the skeleton every server shares: accept, read, write back, one thread per client so a slow player never blocks the rest.

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn echo_ping(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;   // blocks until bytes arrive
        if n == 0 { return Ok(()); }       // 0 bytes = the client closed
        stream.write_all(&buf[..n])?;      // send back exactly what we got
    }
}

let listener = TcpListener::bind("127.0.0.1:0")?;   // port 0 = pick a free one
let (stream, _peer) = listener.accept()?;            // wait for one client
thread::spawn(move || echo_ping(stream));
```

- TCP is a **byte stream**: one `read` may return part of a message or several. Always use the returned length `n`.
- `read` returning `0` means the other side closed. That is how a loop knows to stop.
- `write_all` keeps writing until everything is sent. Plain `write` may stop early.
- `accept()` blocks. To serve many clients at once, hand every accepted stream to its own thread.
- Tests bind `127.0.0.1:0` and use timeouts: no real network, no hanging.

## Your turn

In `src/lib.rs`:

- `echo_ping`: echo until the client closes.
- `send_ping`: connect, send, and read back the same number of bytes.
- `serve`: accept exactly `connections` clients, each on its own thread; join them all at the end.
