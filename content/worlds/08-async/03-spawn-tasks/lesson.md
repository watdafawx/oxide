**Why now:** a download should keep going in the background while the downloader starts the next one; `tokio::spawn` is how each jar gets its own task.

`join!` waits for a fixed set of futures. **`tokio::spawn`** starts a **task**: work that runs on its own, in the background, while you do something else.

- `spawn` takes a future and returns a **`JoinHandle<T>`** immediately.
- `handle.await` waits for the task and gives `Result<T, JoinError>`. `Err` means the task panicked.
- The future must be **`'static`**: it cannot borrow locals, because the task may outlive your function. Use `async move` and give it owned data.
- It must also be **`Send`**: tokio may move it to another thread.

```rust
let name = String::from("jei");
let handle = tokio::spawn(async move {
    format!("hello {name}") // owns `name`
});
let msg: String = handle.await.unwrap();
```

Spawn all the tasks **first**, then await the handles. Awaiting inside the spawn loop makes them run one at a time.

## Your turn

In `src/lib.rs`:

- `spawn_square`: spawn a task that returns `n * n` and return its handle.
- `square_all`: spawn one task per number, then collect the results in order.
- `safe_div`: spawn `a / b` and return `None` if the task panics.
