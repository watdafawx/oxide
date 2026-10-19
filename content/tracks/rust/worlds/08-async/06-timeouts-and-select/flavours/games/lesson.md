**Why now:** a dead mirror can hang a download forever, and the user can press Cancel at any moment. The downloader needs to **stop waiting**: `timeout` for a deadline, `select!` to race two futures.

```rust
use std::time::Duration;
use tokio::time::{sleep, timeout};

// deadline: Err(Elapsed) if download takes longer than 2 s
let result = timeout(Duration::from_secs(2), download("sodium.jar")).await;

// race: the first branch to finish wins, the other future is dropped
let winner = tokio::select! {
    _ = sleep(Duration::from_millis(50)) => "mirror-a",
    _ = sleep(Duration::from_millis(80)) => "mirror-b",
};
```

- `timeout(d, fut).await` is `Ok(value)` or `Err(Elapsed)`. On `Err` the future was **dropped**, so the work stopped.
- `select!` polls several futures and runs the arm of the first one to finish. The losers are dropped.
- A `oneshot::Receiver` is a future too: it completes when someone sends. That makes it a Cancel button.
- Cancellation happens at an `.await`. A future that never awaits cannot be stopped.

## Your turn

In `src/lib.rs`:

- `download_with_timeout`: `Some("done")` if the download finishes in time, else `None`.
- `fastest_mirror`: sleep both mirror times together and return the name of the quicker one.
- `download_or_cancel`: run the download, but return `"cancelled"` if the cancel signal fires first.
