The server's handlers are written, but nothing runs them yet. axum runs on tokio, and Kinetik runs on GPUI's own executor, so the server needs a place of its own to live. This read is the pattern the next quest writes.

**Two executors.** An async function does nothing until an **executor** polls it, the "futures are lazy" rule from the Rust track's Async world. GPUI has its own executor on the main thread; `cx.spawn` uses it. axum and its listener work only on **tokio**, a different executor, and one thread cannot be driven by both.

**A thread of its own.** So the server gets its own thread: `std::thread::spawn(move || ..)` from the Concurrency world. `move` hands the address and the `WebState` over to the new thread, and GPUI's main thread never waits for it.

**A runtime in it.** Inside the thread, `tokio::runtime::Runtime::new()` builds a tokio executor, and `runtime.block_on(future)` runs one future on it, blocking that thread until the future ends. For a server, that is when the app exits. `#[tokio::main]` from the Apps world does these two steps around `main` for you; here `main` already belongs to GPUI.

```rust
fn note_length() -> std::thread::JoinHandle<std::io::Result<usize>> {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(async move {
            let text = tokio::fs::read_to_string("notes.txt").await?;
            Ok(text.len())
        })
    })
}
```

**? inside the closure.** `Runtime::new()` can fail, for example when Windows refuses another thread, and it says so with an `io::Result`. `?` works inside the closure because the closure itself gives back an `io::Result`: the function's return type, `JoinHandle<io::Result<usize>>`, tells Rust so. The same holds inside the `async move` block, which `block_on` hands straight back.

**Errors stay on their side.** A server that cannot start, say because another program has the port, must not take the bar down with it. On its own thread, an error ends only that thread, and it waits in the `JoinHandle` for anyone who calls `join()`.

**For the curious.** `Runtime::new()` starts tokio's multi-threaded runtime, with a worker thread per CPU core. The tests use `Builder::new_current_thread()`, which runs everything on the calling thread: cheaper, and plenty for one small server.

## Your turn

Answer the questions, then start the next quest.
