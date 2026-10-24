Some mod mirrors never answer: the request goes out and nothing comes back. Without a limit, that download waits forever and the launcher shows a spinner until the player gives up. The downloader should give up first, after a deadline.

**A deadline around a future.** `tokio::time::timeout(limit, fut)` wraps any future in a new one. Awaiting it gives `Ok(value)` if `fut` finished within `limit`, or `Err(Elapsed)` if the time ran out first.

```rust
use std::time::Duration;
use tokio::time::timeout;

match timeout(Duration::from_secs(2), fetch_page("sodium")).await {
    Ok(page) => println!("got {page}"),
    Err(_) => println!("the mod site is too slow"),
}
```

**What happens to the slow one.** When the deadline wins, `timeout` drops the wrapped future. Dropping a future stops it for good at the `.await` where it was paused, because nothing will ever poll it again. That is how cancelling works in async Rust: no special call, just a future nobody polls.

**Result into Option.** When the reason doesn't matter, `.ok()` turns `Ok(x)` into `Some(x)` and the `Err(Elapsed)` into `None`, the same trick as two quests ago.

## Your turn

- In `jar_with_deadline`, wait for `fetch_jar(work_ms)` with a limit of `limit_ms`.
- Return `Some` of the answer when it came in time, `None` otherwise.
