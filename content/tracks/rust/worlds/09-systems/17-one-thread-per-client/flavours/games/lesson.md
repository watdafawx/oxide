A Minecraft server that can talk to only one player at a time is no server: the second player waits until the first logs off, and the first may play for an hour. The test server you wrote last quest has exactly that problem, and threads from world 07 fix it.

**`accept` waits.** `listener.accept()` blocks until a client connects, then gives back `(stream, address)`: the new connection and who it is from. A `TcpListener` comes from `TcpListener::bind("127.0.0.1:0")`, which reserves the address and starts listening.

**Try the obvious thing.** The starter's `serve` accepts a client and calls `echo_ping` on it, then accepts the next. But `echo_ping` only returns when its client hangs up. A player who connects and says nothing, AFK in the menu, keeps `serve` stuck inside `echo_ping`, and nobody else gets in. The test does exactly that, and its second player times out.

**A thread per player.** Hand each accepted stream to its own thread with `thread::spawn(move || ...)`. `move` gives the stream to the thread, which now owns it, and the accept loop is free again at once. Each player gets their own `echo_ping`, so a quiet one only blocks themselves.

**Wait for them.** `spawn` gives back a `JoinHandle`. Keep them in a `Vec` and `join` each after the loop, so `serve` returns only when every player is finished, as its doc comment promises.

```rust
let mut workers = Vec::new();
for chunk in 0..3 {
    workers.push(thread::spawn(move || {
        println!("chunk {chunk} generates on its own thread");
    }));
}
for w in workers {
    let _ = w.join();
}
```

**For the curious.** A thread per client is fine for dozens of players. A big server with thousands uses world 08's async tasks instead, which cost far less memory than a thread each.

## Your turn

- In `serve`, give each accepted connection its own thread that runs `echo_ping`.
- Keep every `JoinHandle`, and join them all after the loop.
