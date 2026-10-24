Kinetik's ping stat shows how long a server takes to answer, in milliseconds, and turns red above 100. Kinetik sends ICMP echo packets, the classic ping. This bar times a TCP connection instead: no special rights, and it gets through most firewalls that drop ICMP.

**Time a connection.** Opening a TCP connection takes one round trip to the server and back, which is what ping measures. Note `Instant::now()` before, and `start.elapsed()` after gives the `Duration` in between.

**Never wait forever.** A server that does not answer would block the connect for twenty seconds or more. `TcpStream::connect_timeout(&addr, timeout)` gives up after `timeout`, with an error. `addr` is a `SocketAddr`: an IP address and a port together, like `1.1.1.1:443`.

**No answer is None.** `.ok()?` turns a failed or timed-out connect into `None`, and the bar shows `--`. You do not need the connection itself, only that it opened. `PingSensor` turns a `Some` time into milliseconds. The same timing, around reading a file:

```rust
let start = Instant::now();
let text = fs::read_to_string(path).ok()?;
let took = start.elapsed();
```

**Feel the cost.** `live_sensors` now pings 1.1.1.1 every second, on the same thread that draws the bar. On a slow network the whole bar stalls while it waits. Remember that: three quests from now, the bar moves this work elsewhere.

## Your turn

- In `ping_once`, note the time, then connect to `addr` with the timeout, ending with `None` when it fails.
- Give back the time it took.
- Save. Press Run: ping is real now.
