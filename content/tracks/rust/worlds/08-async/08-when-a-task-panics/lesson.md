The downloader can split a big episode into parts and fetch them side by side. With a bad setting of 0 parts, the division panics. One broken setting should cost one download, not the whole app.

**A panic stays in its task.** When code inside a spawned task panics, tokio catches it at the edge of that task. The task ends, the other tasks and the program keep running, and the panic message is still printed so you can see what went wrong. Threads from world 07 behaved the same way.

**Err tells you.** Awaiting the handle of a task that panicked gives `Err(JoinError)` instead of `Ok(value)`. That is why `handle.await` is a `Result` even when the task itself returns a plain number, and why the last quest wrote `.unwrap()` after it.

```rust
let handle = tokio::spawn(async move { cover_for(&show) }); // may panic
match handle.await {
    Ok(path) => println!("cover at {path}"),
    Err(e) => println!("the cover task failed: {e}"),
}
```

**Keep only the value.** When you don't need the reason, `.ok()` from world 03 turns `Ok(x)` into `Some(x)` and any `Err` into `None`. `e.is_panic()` says whether the task panicked or was cancelled, the other way a task can end early (quest 23).

## Your turn

- In `part_size`, spawn a task that divides `total` by `parts`.
- Await its handle and return the result as an `Option`.
