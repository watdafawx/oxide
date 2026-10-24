`join!` waits for a fixed set of futures inside the current function. The launcher wants each mod download to go on by itself, so it can start more while earlier ones still run. For that, the work becomes a task of its own.

**A task.** `tokio::spawn(fut)` hands a future to the runtime as a new **task**. The runtime starts polling it right away, on any of its worker threads, and `spawn` returns at once. Unlike a plain future, a spawned task runs even if nobody awaits it.

**The handle.** What `spawn` gives back is a `JoinHandle<T>`, a ticket for the task's result. `handle.await` waits until the task is done and gives `Result<T, JoinError>`; `.unwrap()` takes the value out. The next quest is about the `Err`.

**An async block.** `async { .. }` is a future written in place, the way a closure is a function written in place. `async move { .. }` also moves the variables it uses into the future, like the `move` closures that world 07 gave to threads.

```rust
let slug = String::from("sodium");
let icon = tokio::spawn(async move {
    format!("{slug}/icon.png") // the task owns `slug` now
});
let path = icon.await.unwrap(); // "sodium/icon.png"
```

**Why it must own its data.** The task may still run after your function has returned, so it can't borrow your local variables: they would be gone. Rust says the future must be `'static`, which means it holds nothing borrowed that could end early. An owned `String` moved in is fine; a `&str` borrowed from a local is not. It must also be `Send` (world 07), since the runtime may move it to another thread between polls.

**Spawn first, then wait.** To run five downloads at once, spawn all five in one loop and keep the handles, then await the handles in a second loop. Awaiting inside the first loop finishes each task before the next one starts.

## Your turn

- In `spawn_jar_name`, spawn a task that owns `slug` and `version` and builds `"<slug>-<version>.jar"`; return its handle.
- In `all_jar_sizes`, spawn `jar_size(id)` for every id, then await the handles in order.
