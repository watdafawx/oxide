**Why now:** before the downloader gets real HTTP, two mistakes cost beginners hours: forgetting to `.await` a download, and blocking the runtime while "waiting". Both come from how futures work.

An `async fn` call builds a **future**: a paused piece of work. It starts only when something **polls** it, and `.await` is what polls.

```rust
async fn download(jar: &str) -> usize { jar.len() }

let pending = download("sodium.jar"); // nothing has happened yet
// pending is just a value; drop it and the download never runs
let size = pending.await;             // NOW it runs
```

- The compiler warns: `unused implementor of Future that must be used`. Listen to it.
- `tokio::spawn` is the one call that starts work **without** an `.await`: it hands the future to the runtime.
- Async tasks share a few threads. Each task must **give the thread back** at every `.await`.
- `std::thread::sleep`, a blocking file read or a heavy loop does not give it back. Every other task on that thread waits. Use `tokio::time::sleep`, async I/O, or `spawn_blocking` for CPU work.
- Rule of thumb: if a call can take longer than a few milliseconds and has no `.await`, it does not belong in async code.
