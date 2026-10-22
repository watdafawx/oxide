Kinetik's ping stat shows how long a server takes to answer, in milliseconds, and turns red above 100. Kinetik sends ICMP echo packets, the classic ping. This bar times a TCP connection instead: no special rights, and it gets through most firewalls that drop ICMP.

**Time a connection.** `Instant::now()` before, `start.elapsed()` after. Opening a TCP connection takes one round trip to the server, which is what ping measures.

**Never wait forever.** A server that does not answer would block the connect for a long time. `TcpStream::connect_timeout` gives up after `timeout`, with an error:

```rust
TcpStream::connect_timeout(&addr, timeout).ok()?;
```

**No answer is None.** `.ok()?` turns a failed or timed-out connect into `None`, and the bar shows `--`. `PingSensor` turns a `Some` time into milliseconds.

**Feel the cost.** `live_sensors` now pings 1.1.1.1 every second, on the UI thread. On a slow network the whole bar stalls while it waits. Remember that: it is what quest 12 fixes.

## Your turn

- In `ping_once`, note the time, connect with the timeout and `.ok()?`, and give back `Some(start.elapsed())`.
- Save. Press Run: ping is real now.
