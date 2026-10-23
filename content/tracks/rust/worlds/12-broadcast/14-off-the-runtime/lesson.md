The producer clicks "Cut photos" in the HUD while the match is live. The cutting takes a few seconds of solid CPU work, and the scorebug must keep updating the whole time.

**The trap.** Calling `cut_folder` straight from an async task runs it on a runtime thread. It never reaches an `.await`, so the tasks waiting on that thread, the feed among them, wait too. On a single-threaded runtime everything stops.

**Hand it off.** `spawn_blocking` moves a closure to tokio's separate pool of blocking threads. The calling task awaits the result like any other future, and the runtime keeps running in the meantime:

```rust
tokio::task::spawn_blocking(work).await.expect("the work panicked")
```

**Send and 'static.** The work moves to another thread and may outlive the caller's stack, so the closure must be `Send + 'static`: move owned paths in, not references.

**Panics come back.** If the work panics, the handle gives back an error instead of taking the server down. Here that is a bug, so `expect` says so.

## Your turn

- Write `off_the_runtime`: hand `work` to `spawn_blocking` and await its result.
- Save. The test runs a single-threaded runtime, where the difference is easy to see.
