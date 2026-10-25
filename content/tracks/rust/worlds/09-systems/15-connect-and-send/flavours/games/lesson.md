The pinger's first network step: open a connection to a server, send some bytes, read the answer. The standard library's `std::net` has everything, no crate needed. The test runs a tiny stand-in server on `127.0.0.1` that sends back whatever it gets, sometimes slowly, like a laggy server.

**Connect.** `TcpStream::connect(addr)` opens a TCP connection and gives back an `io::Result<TcpStream>`, the `Result` with an `io::Error` from world 04, because the server may be offline. The `TcpStream` is your end of the connection. `SocketAddr` is Rust's type for an IP address plus a port, like `127.0.0.1:25565`.

**Write it all.** `stream.write_all(bytes)` sends every byte. Plain `write` may send only the first part when the network is busy and tell you how much it managed; `write_all` keeps going until all of it is out. The method comes from the `Write` trait, which is why the file has `use std::io::Write`.

**Read exactly.** Remember that TCP is a stream: the answer may arrive in pieces. `stream.read_exact(&mut buf)` keeps reading until `buf` is full, or fails if the connection closes first. `vec![0u8; n]` makes a buffer of `n` zero bytes to read into. Like `write_all`, it comes from a trait, `Read`.

**Byte strings.** The tests send `b"ping 123"`. A `b` in front of a text literal makes it bytes instead of a `&str`: each letter becomes its byte, so `b"hi"` is `[104, 105]`. It is handy for writing test packets.

```rust
let mut stream = TcpStream::connect(addr)?;
stream.write_all(b"status?\n")?;
let mut first = [0u8; 2];          // a fixed-size buffer works too
stream.read_exact(&mut first)?;    // exactly 2 bytes, or an error
```

## Your turn

- In `send_ping`, connect to `addr` and send all of `msg`.
- Then read back exactly `msg.len()` bytes and give them back.
