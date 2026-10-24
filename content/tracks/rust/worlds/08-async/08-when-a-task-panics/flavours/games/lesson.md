The downloader can split a big jar over several mirrors and fetch the parts side by side. With a modpack that lists 0 mirrors, the division panics. One broken modpack file should cost one download, not the whole launcher.

**A panic stays in its task.** When code inside a spawned task panics, tokio catches it at the edge of that task. The task ends, the other tasks and the launcher keep running, and the panic message is still printed so you can see what went wrong. Threads from world 07 behaved the same way.

**Err tells you.** Awaiting the handle of a task that panicked gives `Err(JoinError)` instead of `Ok(value)`. That is why `handle.await` is a `Result` even when the task itself returns a plain number, and why the last quest wrote `.unwrap()` after it.

```rust
let handle = tokio::spawn(async move { icon_for(&slug) }); // may panic
match handle.await {
    Ok(path) => println!("icon at {path}"),
    Err(e) => println!("the icon task failed: {e}"),
}
```

**Keep only the value.** When you don't need the reason, `.ok()` from world 03 turns `Ok(x)` into `Some(x)` and any `Err` into `None`. `e.is_panic()` says whether the task panicked or was cancelled, the other way a task can end early (quest 23).

## Your turn

- In `bytes_per_mirror`, spawn a task that divides `total` by `mirrors`.
- Await its handle and return the result as an `Option`.
