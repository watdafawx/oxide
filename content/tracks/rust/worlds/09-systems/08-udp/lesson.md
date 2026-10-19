**Why now:** servers also answer a UDP query for status and latency, so you will see why a lost reply must time out instead of hanging the pinger.

UDP is TCP's quick cousin: no connection, no retries, no ordering. Each `send_to` is one **datagram** that arrives whole or not at all.

```rust
use std::net::UdpSocket;

let server = UdpSocket::bind("127.0.0.1:0").unwrap();
let addr = server.local_addr().unwrap();

let client = UdpSocket::bind("127.0.0.1:0").unwrap();
client.send_to(b"ping", addr).unwrap();

let mut buf = [0u8; 64];
let (n, from) = server.recv_from(&mut buf).unwrap();
assert_eq!(&buf[..n], b"ping");
server.send_to(b"pong", from).unwrap(); // answer whoever sent it
```

- No `accept`: one socket talks to anyone. `recv_from` tells you **who** sent each datagram.
- `recv_from` returns how many bytes arrived. Only `buf[..n]` is real data.
- Datagrams can be lost, and nothing tells you. **Always set a read timeout** (`set_read_timeout`) or a lost reply hangs your program forever.
- Port `0` means "pick any free port". Tests use it so they never collide.

## Your turn

In `src/lib.rs`:

- `serve_once`: receive one datagram and send it back uppercased.
- `request`: send a message from a fresh socket and return the reply. Time out after 500 ms.
