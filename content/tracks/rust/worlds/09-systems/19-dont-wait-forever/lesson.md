The checker will ask servers that may be switched off. With TCP, a dead server usually means an error. With UDP, a lost question or a lost answer means nothing at all happens, and `request` from the last quest would wait for the rest of time. A checker that hangs is worse than one that says "no answer".

**Silence.** UDP never reports a lost datagram. `recv_from` simply waits for the next datagram to arrive, and if none ever comes, it never returns. The test sends to a socket that never answers, so the starter hangs there.

**A deadline.** `socket.set_read_timeout(Some(time))` gives every later receive a deadline. If nothing arrives in time, `recv_from` returns an `Err` instead, and `?` passes it up like any other failure. `None` means "no deadline", the default, which is why the time goes in a `Some`.

**A length of time.** `std::time::Duration` is Rust's type for a length of time. `Duration::from_millis(500)` is half a second and `Duration::from_secs(3)` is three seconds. Using a type instead of a bare number means nobody has to guess whether `500` was meant in seconds or milliseconds.

**TCP has them too.** A `TcpStream` has `set_read_timeout` and `set_write_timeout`, and `TcpStream::connect_timeout(&addr, time)` connects with a deadline. A server that accepts and then never answers would hang a TCP read just the same, so the boss uses all three.

```rust
let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(3))?;
stream.set_read_timeout(Some(Duration::from_secs(3)))?;
stream.set_write_timeout(Some(Duration::from_secs(3)))?;
```

## Your turn

- In `request`, replace the placeholder line with a read timeout of 500 milliseconds on `socket`, before anything is received.
