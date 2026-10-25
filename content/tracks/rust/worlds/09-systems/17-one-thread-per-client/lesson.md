A server that can talk to only one client at a time is no server: the second one waits until the first is done, and the first may stay connected for an hour. The test server you wrote last quest has exactly that problem, and threads from world 07 fix it.

**`accept` waits.** `listener.accept()` blocks until a client connects, then gives back `(stream, address)`: the new connection and who it is from. A `TcpListener` comes from `TcpListener::bind("127.0.0.1:0")`, which reserves the address and starts listening.

**Try the obvious thing.** The starter's `serve` accepts a client and calls `handle` on it, then accepts the next. But `handle` only returns when its client hangs up. A client that connects and says nothing keeps `serve` stuck inside `handle`, and nobody else gets in. The test does exactly that, and its second client times out.

**A thread per client.** Hand each accepted stream to its own thread with `thread::spawn(move || ...)`. `move` gives the stream to the thread, which now owns it, and the accept loop is free again at once. Each client gets its own `handle`, so a quiet one only blocks itself.

**Wait for them.** `spawn` gives back a `JoinHandle`. Keep them in a `Vec` and `join` each after the loop, so `serve` returns only when every client is finished, as its doc comment promises.

```rust
let mut workers = Vec::new();
for job in 0..3 {
    workers.push(thread::spawn(move || {
        println!("job {job} runs on its own thread");
    }));
}
for w in workers {
    let _ = w.join();
}
```

**For the curious.** A thread per client is fine for dozens of clients. A real server with thousands uses world 08's async tasks instead, which cost far less memory than a thread each.

## Your turn

- In `serve`, give each accepted connection its own thread that runs `handle`.
- Keep every `JoinHandle`, and join them all after the loop.
