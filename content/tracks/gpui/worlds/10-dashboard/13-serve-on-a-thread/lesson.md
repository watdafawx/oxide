The handler, the key check and the address are ready, but no browser can reach them yet. This quest starts the server for real, on the thread and tokio runtime from the read before this one.

**Listen first.** `tokio::net::TcpListener::bind(addr).await` asks Windows for that address and port, and from then on browsers can connect. It fails with an `io::Error` when another program already has the port, so finish it with `?`.

**Then serve.** `axum::serve(listener, router(state)).await` accepts connections and answers each with the router, for as long as the app runs. It finishes only if the server fails, with an `io::Result<()>`: exactly what the thread gives back, so it goes last in the `async move` block, with no `?`.

**The shape you know.** Thread, runtime, `block_on`, then the async work. This sibling only checks that a port can be had and gives back its number:

```rust
fn port_of(addr: SocketAddr) -> std::thread::JoinHandle<std::io::Result<u16>> {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind(addr).await?;
            Ok(listener.local_addr()?.port())
        })
    })
}
```

**Who calls it.** `main.rs` already makes the key on first start, prints the link, calls `serve_dashboard` with `bind_addr(..)` and a `WebState`, and copies a fresh snapshot into the shared state whenever the bar changes.

**It keeps running.** Unlike a GPUI `Task`, dropping a `JoinHandle` does not stop its thread. `main.rs` keeps the handle in `_server` only to show where it lives; the server runs until the app exits.

**Tested on a real port.** The test asks Windows for any free port, starts your server there, and sends a real `GET /api/stats` with the key over TCP.

## Your turn

- In `serve_dashboard`, start a thread that makes a tokio runtime and blocks on an `async move` block.
- In that block, listen on `addr`, then serve `router(state)` on the listener.
- Save. Press Run, then open the printed link in a browser.
