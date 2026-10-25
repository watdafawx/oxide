The producer clicks "Cut photos" in the HUD while the match is live. The cutting takes a few seconds of solid CPU work, and the scorebug must keep updating the whole time.

**The trap.** Calling `cut_folder` straight from an async task runs it on a runtime thread. It never reaches an `.await`, so it never hands the thread back, and the tasks waiting on that thread, the feed among them, wait too. On a single-threaded runtime everything stops.

**Hand it off.** `tokio::task::spawn_blocking(closure)` moves a closure to tokio's separate pool of blocking threads and gives back a handle. The calling task `.await`s the handle like any other future, and the runtime keeps serving every other task in the meantime.

**What comes back.** Awaiting the handle gives a `Result`: `Ok` with the closure's return value, or `Err` if the closure panicked. Reading a big recording file off the runtime looks like this:

```rust
let text = tokio::task::spawn_blocking(move || std::fs::read_to_string(path))
    .await
    .expect("reading the recording panicked");
```

**Send and 'static.** The work moves to another thread and may outlive the caller's stack, so the closure must be `Send + 'static`: move owned values in (the `move` above takes `path` along), not references to the caller's variables. `off_the_runtime` already asks for exactly that in its signature.

**Panics come back.** If the work panics, the handle gives back an error instead of taking the server down. Here a panic is a bug, so `expect` says so.

## Your turn

- In `off_the_runtime`, hand `work` itself to `spawn_blocking`, await the handle, and give back the value.
- Save. The test runs a single-threaded runtime, where the difference is easy to see.
