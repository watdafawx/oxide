**Why now:** a server ping is one TCP connection to a Minecraft server, and this quest is the plain connect, write, read round trip you will reuse in the boss.

A Minecraft server is a **listener** that accepts **connections**. `std::net` gives you both with no crates.

```rust
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // port 0 = OS picks a free one
let addr = listener.local_addr().unwrap();

thread::spawn(move || {
    let (mut conn, _peer) = listener.accept().unwrap();
    let mut buf = [0u8; 5];
    conn.read_exact(&mut buf).unwrap(); // exactly 5 bytes or an error
    conn.write_all(&buf).unwrap();
});

let mut c = TcpStream::connect(addr).unwrap();
c.write_all(b"hello").unwrap();
let mut got = [0u8; 5];
c.read_exact(&mut got).unwrap();          // got == b"hello"
```

- **`read`** may return fewer bytes than asked; `Ok(0)` means the other side closed. **`read_exact`** keeps reading until the buffer is full. **`write_all`** retries until every byte is sent.
- `accept()` blocks. To serve many clients, give each connection **its own thread**.
- TCP is a stream, not messages: bytes arrive in order but your `read` boundaries don't match the sender's `write` boundaries.

## Your turn

In `src/lib.rs`:

- `handle`: echo bytes until the client closes.
- `client_roundtrip`: connect, send, read the same amount back.
- `serve`: accept `connections` clients, each in its own thread, then wait for them.
