The player closes the launcher while five mod downloads run as spawned tasks. Dropping their handles doesn't stop them (quest 14), and a oneshot channel can only tell one task. The launcher needs one stop button that every task sees, and a way to wait until they all really stopped.

**One button, many copies.** A `CancellationToken`, from the `tokio-util` crate, is a shared stop button. `token.clone()` gives another handle to the *same* button, so you give each task its own clone. `token.cancel()` presses it, and every clone sees it at once.

**A future that waits for the press.** `token.cancelled()` is a future that finishes when someone calls `cancel()`. Like the oneshot receiver, it fits into a `select!` branch next to the real work.

```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();
let t = token.clone();
let fetcher = tokio::spawn(async move {
    loop {
        tokio::select! {
            _ = t.cancelled() => return "stopped", // leaves the whole task
            _ = fetch_next_chunk() => {}           // one step done, loop again
        }
    }
});
token.cancel();
```

**A loop around select!.** A worker that keeps going until it is told to stop puts the `select!` inside a `loop`. The work branch does one step and the loop goes round again; the token branch uses `return` to leave the task. Because the token is raced at every wait, the worker stops in the middle of a slow step, not after it. A flag checked between steps would react late.

**Then wait for them.** A clean shutdown is two steps: cancel, then await every handle. Only once all handles are done do you know no task is still writing a jar into the mods folder. The `fetch_add` the workers use to count is the atomic counter from world 07.

## Your turn

- In `spawn_worker`, spawn a task that loops, racing the token against a 5 ms sleep. After each sleep add 1 to `chunks`; on cancel return `"stopped"`.
- In `shutdown`, cancel the token, await every handle and count the `"stopped"` answers.
