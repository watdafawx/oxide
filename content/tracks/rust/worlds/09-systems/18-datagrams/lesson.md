TCP's connect and accept cost a round trip before the first byte, which is a lot for one tiny question. UDP skips that: you send a single packet and maybe get a single packet back. Many status and discovery protocols work this way, so the checker learns it too.

**No connection.** `UdpSocket::bind(addr)` gives you a socket on an address; with port `0`, any free one. There is no `accept` and no `connect`: one socket can send to anyone and receive from anyone.

**One datagram.** `socket.send_to(bytes, addr)` sends the bytes as one **datagram**. It arrives whole, or not at all; it is never cut in two like a TCP stream. Each datagram carries the sender's address with it.

**Who sent it.** `socket.recv_from(&mut buf)` waits for one datagram and gives back `(n, from)`: how many bytes arrived and the sender's address. To answer, `send_to` that `from`.

**Only `buf[..n]`.** The buffer is bigger than most datagrams, so only its first `n` bytes are real. `buf[..n].to_ascii_uppercase()` makes an uppercased copy as a `Vec<u8>`, and `buf[..n].to_vec()` a plain copy you can give back.

```rust
let socket = UdpSocket::bind("127.0.0.1:0")?;
socket.send_to(b"who is there?", "127.0.0.1:9000")?;

let mut buf = [0u8; 256];
let (n, from) = socket.recv_from(&mut buf)?;
println!("{n} bytes from {from}: {:?}", &buf[..n]);
```

## Your turn

- In `serve_once`, receive one datagram and send its bytes back in uppercase to the sender.
- In `request`, bind a fresh socket, send `msg` to `server`, and give back the reply's bytes.
