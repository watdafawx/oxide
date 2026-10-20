**Why now:** besides TCP, Minecraft servers answer a small UDP **query** packet for status. UDP has no connection and no delivery promise: you send a datagram, and either a reply comes or it does not. Your client must not wait forever.

```rust
use std::net::UdpSocket;
use std::time::Duration;

let socket = UdpSocket::bind("127.0.0.1:0")?;                // any free port
socket.set_read_timeout(Some(Duration::from_millis(500)))?;  // never wait forever

socket.send_to(b"status?", "127.0.0.1:25565")?;               // fire
let mut buf = [0u8; 512];
let (n, from) = socket.recv_from(&mut buf)?;                  // wait for the reply
println!("{n} bytes from {from}");
```

- No `connect`, no `accept`: one socket sends to and receives from anyone.
- Each `recv_from` gives **one whole datagram** plus the sender's address. Reply to that address.
- Datagrams can be lost. Without a read timeout a lost reply hangs the program. With one, `recv_from` returns an `Err`.
- `buf[..n].to_ascii_uppercase()` uppercases a byte slice into a new `Vec<u8>`; `buf[..n].to_vec()` copies the real bytes into a `Vec<u8>` you can return.
- Tests bind `127.0.0.1:0` and use timeouts: no real network.

## Your turn

In `src/lib.rs`:

- `answer_query`: receive one datagram, send it back uppercased to its sender.
- `send_query`: send a message to a server from a fresh socket and return the reply, giving up after 500 ms.
