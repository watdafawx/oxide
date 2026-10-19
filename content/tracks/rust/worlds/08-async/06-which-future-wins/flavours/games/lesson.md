**Why now:** next you wire the downloader to a Cancel button and to per-mirror timeouts. Both rely on one rule: **dropping a future cancels it**.

```rust
tokio::select! {
    _ = cancel_button.recv() => println!("user cancelled"),
    r = download(url)        => println!("finished: {r:?}"),
}
// whichever arm lost has been DROPPED here
```

- A future only makes progress while someone polls it. Drop it and it never runs again.
- A dropped download stops at its **next `.await`**. Anything it had already written (a half-written `.part` file) stays on disk: clean it up yourself.
- `timeout(d, fut)` is just a `select!` between `fut` and a timer. `Err(Elapsed)` means the deadline won and `fut` was dropped.
- `tokio::spawn`ed tasks are **not** cancelled when you drop their `JoinHandle`. They keep running. To stop a task, abort it or send it a cancel signal (next quests).
- If a branch must not be dropped half-way, it needs to be cancellation safe. Check the docs of the call you race.
