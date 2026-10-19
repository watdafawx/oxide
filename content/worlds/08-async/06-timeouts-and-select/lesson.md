Servers hang. A good app never waits forever, and a good app can be told to stop.

- **`timeout(dur, fut).await`** returns `Ok(value)` if the future finishes in time, `Err(Elapsed)` if not. The future is dropped (cancelled) on timeout.
- **`select!`** waits on several futures and runs the branch of the **first** one to finish. The others are dropped.
- A classic use: `select!` between "the work" and "a cancel signal".

```rust
use std::time::Duration;
use tokio::time::{sleep, timeout};

async fn slow() -> u32 {
    sleep(Duration::from_secs(10)).await;
    1
}

let r = timeout(Duration::from_millis(50), slow()).await;
assert!(r.is_err()); // gave up after 50 ms

let msg = tokio::select! {
    _ = sleep(Duration::from_millis(10)) => "fast",
    _ = sleep(Duration::from_millis(500)) => "slow",
};
assert_eq!(msg, "fast");
```

Dropping a future stops it at its next `.await`. That is what makes cancellation cheap in async Rust.

## Your turn

In `src/lib.rs` (`work` is given, it sleeps then returns `"done"`):

- `with_deadline`: `Some(result)` if `work` finishes within the limit, otherwise `None`.
- `first_of`: sleep `a_ms` vs `b_ms`, return `"a"` or `"b"`.
- `work_or_cancel`: `select!` between the work and the `cancel` receiver.
