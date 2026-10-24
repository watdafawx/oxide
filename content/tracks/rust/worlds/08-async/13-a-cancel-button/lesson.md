A big episode is downloading and the user presses Cancel. The download should stop now, not when it ends. You know how to race two futures; what's missing is a future that finishes the moment the button is pressed.

**A channel for one message.** `tokio::sync::oneshot::channel()` makes a channel that carries exactly one value. The sender's `send(x)` is a plain call, not async: it can never be full. The button handler keeps the sender, and the download keeps the receiver.

**The receiver is a future.** Awaiting a `oneshot::Receiver` waits until the message arrives, so it fits straight into a `select!` branch next to the work. When the button wins, `select!` drops the download, which stops at its next `.await`.

```rust
use tokio::sync::oneshot;

let (press, mut stop) = oneshot::channel::<()>();
let result = tokio::select! {
    _ = &mut stop => "stopped",
    _ = refresh_feed() => "refreshed",
};
```

**Borrow it with &mut.** A `select!` branch takes ownership of its future. Writing `&mut stop` lends the receiver instead, so the function still owns it after the `select!`, for example to wait on it again in a loop. Using it by value also works when it is the last use; `&mut` is the habit that always works.

**Keep the winner's answer.** If the download wins, its own answer should come back, so its branch binds the value, `answer = download(..)`, and gives `answer` back after `=>`.

## Your turn

- In `download_or_cancel`, race the borrowed `cancel` receiver against `download(work_ms)` with `select!`.
- Give back `"cancelled"` when the button wins, and the download's answer when it finishes first.
