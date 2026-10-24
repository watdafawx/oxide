A feed lists 40 episodes. Spawning 40 downloads at once works, but the server may slow down or block an app that opens 40 connections, and 40 downloads share one internet line anyway. Four at a time is usually faster and much kinder to the server.

**Spawn all, let them queue.** Keep quest 07's shape, one task per episode, spawned first and awaited after. Add one thing every task must get before it starts downloading, of which only four exist. A task that can't get one waits at that line, without holding a thread.

**A box of permits.** A `tokio::sync::Semaphore` is that thing: `Semaphore::new(4)` holds 4 **permits**. `acquire().await` takes one, or waits until one is free. The permit is an ordinary value, and dropping it puts it back in the box, which wakes the next waiting task.

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

let permits = Arc::new(Semaphore::new(3));
let mine = Arc::clone(&permits);
tokio::spawn(async move {
    let _permit = mine.acquire_owned().await.unwrap(); // waits if 3 are taken
    refresh_feed("daily").await;
}); // _permit dropped here: the next task may go
```

**Owned, for a task.** The plain `acquire()` borrows the semaphore, and a spawned task can't hold borrows (quest 07). `acquire_owned()` is called on an `Arc<Semaphore>` and gives a permit that owns its own `Arc`, so the task can keep it. It returns a `Result` that is only `Err` if someone closed the semaphore, so `.unwrap()` is fine here.

**Name it, or it's gone.** `let _ = permits.acquire_owned().await;` drops the permit at the `;`, since `_` is not a variable, and your limit silently stops working. `let _permit = ..` is a real variable whose name starts with `_` only to say "not read", so it lives until the end of the block.

## Your turn

- In `download_limited`, make a semaphore with `limit` permits in an `Arc`.
- Spawn one task per episode that takes its own permit, keeps it while it runs `fake_download(&tracker)`, and lets it go at the end.
- Await every handle.
